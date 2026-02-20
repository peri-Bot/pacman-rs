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

use crate::entities::{Ghost, PacMan};
use crate::maze::Maze;

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
}
