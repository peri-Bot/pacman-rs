// game/src/state.rs
//
// Top-level game state: aggregates the maze, entities, and game metadata.
// Exposed to JavaScript via an opaque wasm-bindgen wrapper.
//
// # Architecture: opaque wrapper pattern
// wasm-bindgen can't export structs that contain `Vec`, nested structs, or
// other complex types as fields. So we split into two layers:
//
// 1. `GameStateInner` — plain Rust struct with all the data. Not wasm-bindgen.
//    This is where all game logic lives. Unit tests operate on this.
//
// 2. `GameState` — an opaque `#[wasm_bindgen]` struct wrapping `GameStateInner`.
//    It exposes methods to JS. JS never sees the inner fields directly;
//    it calls `to_js()` to get a serialized snapshot via serde-wasm-bindgen.
//
// This pattern keeps the internal data model flexible while providing
// a clean, stable API to JavaScript.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::entities::{Direction, Ghost, GhostMode, PacMan};
use crate::maze::{CellType, Maze};

// ─── Game Mode ──────────────────────────────────────────────────────────────

/// Determines whether ghosts are AI-controlled or player-controlled.
///
/// - `Classic`: Single-player. Ghosts use AI (Blinky chases, Pinky ambushes, etc.)
/// - `PvP`: Local 1v1. Player 1 is Pac-Man, Player 2 controls the ghosts.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum GameMode {
    Classic,
    PvP,
}

// ─── Game Phase ─────────────────────────────────────────────────────────────

/// The lifecycle phase of the game.
///
/// ```text
/// Ready → Playing ←→ Paused
///           ↓
///        GameOver
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum GamePhase {
    /// Waiting for the player to press start (the "READY!" screen)
    Ready,
    /// Active gameplay
    Playing,
    /// Paused (only in single-player Classic mode typically)
    Paused,
    /// Game over — all lives lost
    GameOver,
}

// ─── Inner Game State (pure Rust) ───────────────────────────────────────────

/// The complete state of a Pac-Man game.
///
/// # Ownership model
/// `GameStateInner` *owns* everything: the maze, Pac-Man, and the ghost vec.
/// When dropped, all nested data is freed automatically (RAII — no GC needed).
/// There's no shared ownership or reference counting because only one
/// `GameStateInner` exists at a time.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameStateInner {
    pub mode: GameMode,
    pub phase: GamePhase,
    pub maze: Maze,
    pub pacman: PacMan,
    pub ghosts: Vec<Ghost>,
    pub dots_remaining: usize,
    pub level: u32,
    pub global_timer: f64,
    pub frightened_timer: f64,
}

impl GameStateInner {
    /// Create a new game with the given mode.
    pub fn new(mode: GameMode) -> Self {
        let maze = Maze::new();
        let dots = maze.dots_remaining();

        GameStateInner {
            mode,
            phase: GamePhase::Ready,
            maze,
            pacman: PacMan::new(),
            ghosts: Ghost::create_all(),
            dots_remaining: dots,
            level: 1,
            global_timer: 0.0,
            frightened_timer: 0.0,
        }
    }

    /// Get the mode as a string.
    pub fn mode_str(&self) -> &'static str {
        match self.mode {
            GameMode::Classic => "classic",
            GameMode::PvP => "pvp",
        }
    }

    /// Get the phase as a string.
    pub fn phase_str(&self) -> &'static str {
        match self.phase {
            GamePhase::Ready => "ready",
            GamePhase::Playing => "playing",
            GamePhase::Paused => "paused",
            GamePhase::GameOver => "gameover",
        }
    }

    /// Check if the game is over (no lives remaining).
    pub fn is_game_over(&self) -> bool {
        self.pacman.lives == 0
    }

    /// Check if the current level is complete (all dots eaten).
    pub fn is_level_complete(&self) -> bool {
        self.dots_remaining == 0
    }

    /// Advance the game state by `dt` seconds.
    pub fn tick(&mut self, dt: f64) {
        if self.phase != GamePhase::Playing {
            return;
        }

        self.update_timers(dt);

        const PAC_SPEED: f64 = 11.0; // Tiles per second
        let pac_dist = PAC_SPEED * dt;

        self.update_pacman(pac_dist);
        self.update_ghosts(dt);
        self.check_collisions();
    }

    fn update_timers(&mut self, dt: f64) {
        let old_frightened = self.frightened_timer > 0.0;

        if self.frightened_timer > 0.0 {
            self.frightened_timer -= dt;
            if self.frightened_timer <= 0.0 {
                self.frightened_timer = 0.0;
            }
        } else {
            self.global_timer += dt;
        }

        let cycle_time = self.global_timer % 27.0;
        let global_mode = if cycle_time < 7.0 {
            GhostMode::Scatter
        } else {
            GhostMode::Chase
        };

        let mut toggle_reverse = false;

        // If we just recovered from frightened, we restore modes
        if old_frightened && self.frightened_timer <= 0.0 {
            for ghost in &mut self.ghosts {
                if ghost.mode == GhostMode::Frightened {
                    ghost.mode = global_mode;
                }
            }
        } else if self.frightened_timer <= 0.0 {
            // Apply global mode changes
            for ghost in &mut self.ghosts {
                if (ghost.mode == GhostMode::Chase || ghost.mode == GhostMode::Scatter)
                    && ghost.mode != global_mode
                {
                    ghost.mode = global_mode;
                    toggle_reverse = true;
                }
            }
        }

        if toggle_reverse {
            for ghost in &mut self.ghosts {
                if ghost.mode == GhostMode::Chase || ghost.mode == GhostMode::Scatter {
                    ghost.direction = ghost.direction.opposite();
                }
            }
        }
    }

    fn get_ghost_target(
        ghost: &Ghost,
        pac_pos: &crate::entities::Position,
        pac_dir: Direction,
        blinky_pos: &crate::entities::Position,
        global_timer: f64,
    ) -> (isize, isize) {
        use crate::entities::GhostType;
        match ghost.mode {
            GhostMode::Scatter => match ghost.ghost_type {
                GhostType::Blinky => (25, -3),
                GhostType::Pinky => (2, -3),
                GhostType::Inky => (27, 31),
                GhostType::Clyde => (0, 31),
            },
            GhostMode::Chase => match ghost.ghost_type {
                GhostType::Blinky => {
                    let (c, r) = pac_pos.to_grid();
                    (c as isize, r as isize)
                }
                GhostType::Pinky => {
                    let (c, r) = pac_pos.to_grid();
                    let (dx, dy) = pac_dir.to_vector();
                    (c as isize + dx as isize * 4, r as isize + dy as isize * 4)
                }
                GhostType::Inky => {
                    let (pc, pr) = pac_pos.to_grid();
                    let (dx, dy) = pac_dir.to_vector();
                    let pivot_c = pc as isize + dx as isize * 2;
                    let pivot_r = pr as isize + dy as isize * 2;
                    let (bc, br) = blinky_pos.to_grid();
                    let vec_c = pivot_c - bc as isize;
                    let vec_r = pivot_r - br as isize;
                    (pivot_c + vec_c, pivot_r + vec_r)
                }
                GhostType::Clyde => {
                    let (c, r) = pac_pos.to_grid();
                    let (gc, gr) = ghost.position.to_grid();
                    let dist_sq =
                        (c as isize - gc as isize).pow(2) + (r as isize - gr as isize).pow(2);
                    if dist_sq > 64 {
                        (c as isize, r as isize)
                    } else {
                        (0, 31)
                    }
                }
            },
            GhostMode::Frightened => {
                // Pseudo-random wander
                let seed = global_timer * 10.0 + ghost.position.x * 3.0;
                ((seed as isize) % 28, (seed as isize * 7) % 31)
            }
            GhostMode::Eaten => {
                (14, 11) // House entrance
            }
        }
    }

    fn update_ghosts(&mut self, dt: f64) {
        // Different speeds depending on mode
        let base_speed = 9.0;

        let pac_pos = self.pacman.position.clone();
        let pac_dir = self.pacman.direction;
        let mut blinky_pos = self.ghosts[0].position.clone();
        for g in &self.ghosts {
            if g.ghost_type == crate::entities::GhostType::Blinky {
                blinky_pos = g.position.clone();
            }
        }

        for ghost in &mut self.ghosts {
            let speed = match ghost.mode {
                GhostMode::Frightened => base_speed * 0.5,
                GhostMode::Eaten => base_speed * 2.0,
                _ => base_speed,
            };
            let dist = speed * dt;

            // If Eaten and reaches house, revive
            if ghost.mode == GhostMode::Eaten {
                let (c, r) = ghost.position.to_grid();
                if c == 14 && r == 11 {
                    ghost.mode = if self.frightened_timer <= 0.0 {
                        if (self.global_timer % 27.0) < 7.0 {
                            GhostMode::Scatter
                        } else {
                            GhostMode::Chase
                        }
                    } else {
                        GhostMode::Chase // Or wait in house
                    };
                }
            }

            let is_player = self.mode == GameMode::PvP
                && ghost.ghost_type == crate::entities::GhostType::Blinky
                && ghost.mode != GhostMode::Eaten
                && ghost.mode != GhostMode::Frightened;

            if is_player && ghost.next_direction != ghost.direction {
                if ghost.next_direction == ghost.direction.opposite() {
                    ghost.direction = ghost.next_direction;
                } else {
                    let cx = ghost.position.x.round();
                    let cy = ghost.position.y.round();
                    let is_near_center = (ghost.position.x - cx).abs() <= dist
                        && (ghost.position.y - cy).abs() <= dist;
                    if is_near_center {
                        let (dx, dy) = ghost.next_direction.to_vector();
                        if self.maze.is_walkable(cx + dx, cy + dy) {
                            ghost.position.x = cx;
                            ghost.position.y = cy;
                            ghost.direction = ghost.next_direction;
                        }
                    }
                }
            }

            let (dx, dy) = ghost.direction.to_vector();
            let mut new_x = ghost.position.x + dx * dist;
            let mut new_y = ghost.position.y + dy * dist;

            let cx = ghost.position.x.round();
            let cy = ghost.position.y.round();

            let crossed_center = match ghost.direction {
                Direction::Right => ghost.position.x < cx && new_x >= cx,
                Direction::Left => ghost.position.x > cx && new_x <= cx,
                Direction::Down => ghost.position.y < cy && new_y >= cy,
                Direction::Up => ghost.position.y > cy && new_y <= cy,
            };

            if is_player {
                let next_cx = cx + dx;
                let next_cy = cy + dy;
                if (crossed_center
                    || (dx > 0.0 && new_x > cx)
                    || (dx < 0.0 && new_x < cx)
                    || (dy > 0.0 && new_y > cy)
                    || (dy < 0.0 && new_y < cy))
                    && !self.maze.is_walkable(next_cx, next_cy)
                {
                    new_x = cx;
                    new_y = cy;
                }
            } else if crossed_center {
                let target = Self::get_ghost_target(
                    ghost,
                    &pac_pos,
                    pac_dir,
                    &blinky_pos,
                    self.global_timer,
                );

                let possible_dirs = [
                    Direction::Up,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                ];
                let mut best_dir = ghost.direction;
                let mut min_dist_sq = f64::MAX;

                let mut options = 0;

                for &dir in &possible_dirs {
                    if dir == ghost.direction.opposite() {
                        continue; // No reversing
                    }
                    let (tdx, tdy) = dir.to_vector();
                    let tx = cx + tdx;
                    let ty = cy + tdy;

                    if self.maze.is_walkable(tx, ty) || ghost.mode == GhostMode::Eaten {
                        options += 1;
                        let dist_sq =
                            (tx - target.0 as f64).powi(2) + (ty - target.1 as f64).powi(2);
                        if dist_sq < min_dist_sq {
                            min_dist_sq = dist_sq;
                            best_dir = dir;
                        }
                    }
                }

                if options == 0 {
                    best_dir = ghost.direction.opposite(); // Fallback if dead end
                }

                // Snap to center and switch to new direction
                new_x = cx;
                new_y = cy;
                ghost.direction = best_dir;
            }

            ghost.position.x = new_x;
            ghost.position.y = new_y;

            // Tunnel wrap
            let width = self.maze.width as f64;
            if ghost.position.x < -0.5 {
                ghost.position.x += width;
            } else if ghost.position.x >= width - 0.5 {
                ghost.position.x -= width;
            }
        }
    }

    fn update_pacman(&mut self, dist: f64) {
        let pac = &mut self.pacman;

        // 1. Can we turn to `next_direction`?
        if pac.next_direction != pac.direction {
            // Immediate reverse is always allowed
            if pac.next_direction == pac.direction.opposite() {
                pac.direction = pac.next_direction;
            } else {
                // To turn 90 degrees, we must be close to the tile center
                let cx = pac.position.x.round();
                let cy = pac.position.y.round();

                // Are we close enough to the center to turn?
                let is_near_center =
                    (pac.position.x - cx).abs() <= dist && (pac.position.y - cy).abs() <= dist;

                if is_near_center {
                    // Peek at the tile in the next_direction
                    let (dx, dy) = pac.next_direction.to_vector();
                    let target_x = cx + dx;
                    let target_y = cy + dy;

                    if self.maze.is_walkable(target_x, target_y) {
                        // Snap to center and turn
                        pac.position.x = cx;
                        pac.position.y = cy;
                        pac.direction = pac.next_direction;
                    }
                }
            }
        }

        // 2. Move forward in the current `direction`
        let (dx, dy) = pac.direction.to_vector();
        let mut new_x = pac.position.x + dx * dist;
        let mut new_y = pac.position.y + dy * dist;

        // 3. Wall collision: if we passed the center and the next tile is a wall, clamp to center
        let cx = pac.position.x.round();
        let cy = pac.position.y.round();

        let next_cx = cx + dx;
        let next_cy = cy + dy;

        // Check if we crossed the center boundary towards a wall
        let crossed_center = match pac.direction {
            Direction::Right => pac.position.x < cx && new_x >= cx,
            Direction::Left => pac.position.x > cx && new_x <= cx,
            Direction::Down => pac.position.y < cy && new_y >= cy,
            Direction::Up => pac.position.y > cy && new_y <= cy,
        };

        if (crossed_center
            || (dx > 0.0 && new_x > cx)
            || (dx < 0.0 && new_x < cx)
            || (dy > 0.0 && new_y > cy)
            || (dy < 0.0 && new_y < cy))
            && !self.maze.is_walkable(next_cx, next_cy)
        {
            // Clamp to center
            new_x = cx;
            new_y = cy;
        }

        pac.position.x = new_x;
        pac.position.y = new_y;

        // Wrap around using modulo (maze width is 28)
        let width = self.maze.width as f64;
        if pac.position.x < -0.5 {
            pac.position.x += width;
        } else if pac.position.x >= width - 0.5 {
            pac.position.x -= width;
        }
    }

    fn check_collisions(&mut self) {
        // Collect dot/pellet collisions
        let (col, row) = self.pacman.position.to_grid();
        if let Some(cell) = self.maze.get_cell(row, col) {
            match cell {
                CellType::Dot => {
                    self.pacman.score += 10;
                    self.dots_remaining -= 1;
                    self.maze.cells[row][col] = CellType::Empty;
                }
                CellType::PowerPellet => {
                    self.pacman.score += 50;
                    self.dots_remaining -= 1;
                    self.maze.cells[row][col] = CellType::Empty;
                    // Frighten ghosts
                    for ghost in &mut self.ghosts {
                        if ghost.mode != GhostMode::Eaten {
                            ghost.mode = GhostMode::Frightened;
                            // Reversing direction when frightened is classic behavior
                            ghost.direction = ghost.direction.opposite();
                        }
                    }
                }
                _ => {}
            }
        }

        // Ghost collisions
        for ghost in &mut self.ghosts {
            let dx = self.pacman.position.x - ghost.position.x;
            let dy = self.pacman.position.y - ghost.position.y;
            let dist_sq = dx * dx + dy * dy;

            if dist_sq < 0.25 {
                // Collision distance (radius 0.5)
                match ghost.mode {
                    GhostMode::Frightened => {
                        self.pacman.score += 200; // Base score for eating ghost
                        ghost.mode = GhostMode::Eaten;
                    }
                    GhostMode::Chase | GhostMode::Scatter => {
                        if self.pacman.lives > 0 {
                            self.pacman.lives -= 1;
                        }
                        if self.pacman.lives == 0 {
                            self.phase = GamePhase::GameOver;
                        } else {
                            self.phase = GamePhase::Paused; // Wait for respawn
                        }
                    }
                    GhostMode::Eaten => {}
                }
            }
        }

        if self.dots_remaining == 0 {
            self.phase = GamePhase::Paused; // Level complete wait state
        }
    }
}

// ─── WASM-exposed wrapper ───────────────────────────────────────────────────

/// Opaque wrapper exposed to JavaScript via wasm-bindgen.
///
/// # Why a wrapper?
/// wasm-bindgen requires `#[wasm_bindgen]` on the struct itself, but it
/// can't handle structs with `Vec` or nested struct fields. By wrapping
/// `GameStateInner` in an opaque struct, wasm-bindgen sees it as a simple
/// pointer (u32 index into a slab). JS never accesses fields directly —
/// it calls methods to get/set data.
///
/// # Ownership note
/// `GameState` *owns* the `GameStateInner`. The `#[wasm_bindgen]` macro
/// generates code that stores the value in a global slab on the Rust side.
/// When JS drops the `GameState` object (garbage collection), wasm-bindgen
/// calls Rust's `Drop` to free the inner data.
#[wasm_bindgen]
pub struct GameState {
    inner: GameStateInner,
}

#[wasm_bindgen]
impl GameState {
    /// Create a new game state.
    ///
    /// # Arguments
    /// * `mode` — `"classic"` or `"pvp"` (case-insensitive)
    ///
    /// # Why `&str` and not `GameMode`?
    /// wasm-bindgen cannot pass Rust enums directly across the WASM boundary.
    /// We accept a string and parse it inside Rust, keeping the type-safe
    /// `GameMode` enum as the internal representation.
    #[wasm_bindgen(constructor)]
    pub fn new(mode: &str) -> GameState {
        let game_mode = match mode.to_lowercase().as_str() {
            "classic" => GameMode::Classic,
            "pvp" => GameMode::PvP,
            _ => panic!("Invalid game mode: '{}'. Use 'classic' or 'pvp'.", mode),
        };

        GameState {
            inner: GameStateInner::new(game_mode),
        }
    }

    /// Sets the intended next direction for Pac-Man.
    pub fn set_direction(&mut self, dir: &str) {
        use crate::entities::Direction;
        let direction = match dir.to_lowercase().as_str() {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => return, // Ignore invalid input
        };

        self.inner.pacman.next_direction = direction;

        // Start game on first input
        if self.inner.phase == GamePhase::Ready {
            self.inner.phase = GamePhase::Playing;
        }
    }

    /// Sets the intended next direction for Player 2's Ghost (Blinky).
    pub fn set_player2_direction(&mut self, dir: &str) {
        use crate::entities::Direction;
        let direction = match dir.to_lowercase().as_str() {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => return,
        };

        if !self.inner.ghosts.is_empty() {
            self.inner.ghosts[0].next_direction = direction;
        }

        // Start game on first input
        if self.inner.phase == GamePhase::Ready {
            self.inner.phase = GamePhase::Playing;
        }
    }

    /// Advance game logic by delta time (in milliseconds)
    pub fn tick(&mut self, dt_ms: f64) {
        let dt_seconds = dt_ms / 1000.0;
        self.inner.tick(dt_seconds);
    }

    /// Serialize the entire game state to a JS object.
    ///
    /// `serde-wasm-bindgen` converts the Rust struct tree into a plain JS
    /// object. The resulting `JsValue` can be read directly by Vue's
    /// reactive system.
    ///
    /// # Performance note
    /// Full serialization on every call (~868 maze cells + entities).
    /// Fine for debugging; will switch to delta-based updates for 60fps.
    pub fn to_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner).unwrap()
    }

    /// Get the current game mode as a string.
    pub fn get_mode(&self) -> String {
        self.inner.mode_str().to_string()
    }

    /// Get the current game phase as a string.
    pub fn get_phase(&self) -> String {
        self.inner.phase_str().to_string()
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

/// Tests operate on `GameStateInner` directly (no WASM needed).
/// The `GameState` wrapper just delegates, so testing inner is sufficient.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::GhostType;
    use crate::maze::{MAZE_HEIGHT, MAZE_WIDTH};

    // Helper to build inner state (avoids repeating the match logic in tests)
    fn classic() -> GameStateInner {
        GameStateInner::new(GameMode::Classic)
    }

    fn pvp() -> GameStateInner {
        GameStateInner::new(GameMode::PvP)
    }

    #[test]
    fn new_classic_game_has_correct_mode() {
        let state = classic();
        assert_eq!(state.mode, GameMode::Classic);
        assert_eq!(state.mode_str(), "classic");
    }

    #[test]
    fn new_pvp_game_has_correct_mode() {
        let state = pvp();
        assert_eq!(state.mode, GameMode::PvP);
        assert_eq!(state.mode_str(), "pvp");
    }

    #[test]
    fn new_game_starts_in_ready_phase() {
        let state = classic();
        assert_eq!(state.phase, GamePhase::Ready);
        assert_eq!(state.phase_str(), "ready");
    }

    #[test]
    fn new_game_has_correct_maze_size() {
        let state = classic();
        assert_eq!(state.maze.width, MAZE_WIDTH);
        assert_eq!(state.maze.height, MAZE_HEIGHT);
    }

    #[test]
    fn new_game_has_four_ghosts() {
        let state = classic();
        assert_eq!(state.ghosts.len(), 4);
    }

    #[test]
    fn new_game_ghosts_are_blinky_pinky_inky_clyde() {
        let state = classic();
        let types: Vec<GhostType> = state.ghosts.iter().map(|g| g.ghost_type).collect();
        assert_eq!(
            types,
            vec![
                GhostType::Blinky,
                GhostType::Pinky,
                GhostType::Inky,
                GhostType::Clyde,
            ]
        );
    }

    #[test]
    fn new_game_has_dots() {
        let state = classic();
        assert!(state.dots_remaining > 0);
    }

    #[test]
    fn new_game_starts_at_level_1() {
        let state = classic();
        assert_eq!(state.level, 1);
    }

    #[test]
    fn new_game_pacman_has_3_lives() {
        let state = classic();
        assert_eq!(state.pacman.lives, 3);
        assert_eq!(state.pacman.score, 0);
    }

    #[test]
    fn is_game_over_with_no_lives() {
        let mut state = classic();
        state.pacman.lives = 0;
        assert!(state.is_game_over());
    }

    #[test]
    fn is_level_complete_with_no_dots() {
        let mut state = classic();
        state.dots_remaining = 0;
        assert!(state.is_level_complete());
    }

    // Test the WASM wrapper's string parsing
    #[test]
    fn wasm_wrapper_classic() {
        let gs = GameState::new("classic");
        assert_eq!(gs.get_mode(), "classic");
        assert_eq!(gs.get_phase(), "ready");
    }

    #[test]
    fn wasm_wrapper_pvp_case_insensitive() {
        let gs = GameState::new("PVP");
        assert_eq!(gs.get_mode(), "pvp");
    }

    #[test]
    #[should_panic(expected = "Invalid game mode")]
    fn wasm_wrapper_invalid_mode_panics() {
        GameState::new("invalid");
    }

    #[test]
    fn test_ghost_movement_out_of_spawn() {
        let mut gs = GameStateInner::new(GameMode::Classic);
        gs.phase = GamePhase::Playing;
        
        let initial_y = gs.ghosts[0].position.y;
        
        // Tick a few frames (0.016 seconds each)
        for _ in 0..10 {
            gs.tick(0.016);
        }
        
        println!("Ghost 0 after 10 ticks: x: {}, y: {}, dir: {:?}", gs.ghosts[0].position.x, gs.ghosts[0].position.y, gs.ghosts[0].direction);
        
        assert!(gs.ghosts[0].position.y < initial_y, "Blinky should move up from start");
        
        // Tick until he reaches row 10
        for _ in 0..50 {
            gs.tick(0.016);
        }
        
        println!("Ghost 0 after 60 ticks: x: {}, y: {}, dir: {:?}", gs.ghosts[0].position.x, gs.ghosts[0].position.y, gs.ghosts[0].direction);
    }
}
