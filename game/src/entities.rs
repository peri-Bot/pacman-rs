// game/src/entities.rs
//
// Game entities: Pac-Man and Ghosts.
//
// These structs represent the *state* of each entity.
// Movement logic and AI will be added in later phases.

use serde::{Deserialize, Serialize};

// ─── Direction ──────────────────────────────────────────────────────────────

/// The four cardinal directions an entity can face / move.
///
/// # Why derive `Copy`?
/// `Direction` is a small enum (fits in a single byte). Deriving `Copy`
/// means it's passed by value automatically — no need for `clone()`.
/// When you write `let d = some_entity.direction;`, Rust copies the byte
/// instead of moving ownership. This is only safe for small, simple types.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// ─── Position ───────────────────────────────────────────────────────────────

/// A 2D position using `f64` for smooth sub-cell movement.
///
/// # Why f64 and not usize?
/// Pac-Man moves smoothly between grid cells for fluid animation.
/// During rendering, JS reads these fractional coordinates to position
/// sprites. For collision detection, we round to the nearest grid cell:
///
/// ```text
/// let grid_row = position.y.round() as usize;
/// let grid_col = position.x.round() as usize;
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }

    /// Convert to grid coordinates (column, row).
    pub fn to_grid(&self) -> (usize, usize) {
        (self.x.round() as usize, self.y.round() as usize)
    }
}

// ─── Pac-Man ────────────────────────────────────────────────────────────────

/// The player-controlled Pac-Man entity.
///
/// # Ownership note
/// `PacMan` *owns* its `Position` — the position is stored inline
/// in the struct (no pointer indirection). When `PacMan` is dropped,
/// the `Position` is dropped with it automatically. This is Rust's
/// ownership model in action: each value has exactly one owner.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PacMan {
    pub position: Position,
    pub direction: Direction,
    pub next_direction: Direction,
    pub lives: u8,
    pub score: u32,
}

impl PacMan {
    /// Create Pac-Man at the classic starting position (row 23, col 14).
    pub fn new() -> Self {
        PacMan {
            position: Position::new(14.0, 23.0),
            direction: Direction::Left,
            next_direction: Direction::Left,
            lives: 3,
            score: 0,
        }
    }
}

impl Default for PacMan {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Ghost types ────────────────────────────────────────────────────────────

/// The four classic ghost personalities.
///
/// Each ghost type has a different AI targeting strategy in Classic mode:
/// - **Blinky** (red): Directly chases Pac-Man's current tile
/// - **Pinky** (pink): Targets 4 tiles ahead of Pac-Man
/// - **Inky** (cyan): Complex targeting using Blinky's position
/// - **Clyde** (orange): Chases when far, scatters when close
///
/// In PvP mode, Player 2 controls all ghosts directly, so these
/// personalities only matter for Classic mode AI.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum GhostType {
    Blinky,
    Pinky,
    Inky,
    Clyde,
}

// ─── Ghost modes ────────────────────────────────────────────────────────────

/// The behavioral state of a ghost.
///
/// # State machine
/// ```text
///  Scatter ←→ Chase   (alternates on a timer)
///       ↓       ↓
///     Frightened       (when Pac-Man eats a power pellet)
///       ↓
///      Eaten           (ghost returns to ghost house)
///       ↓
///     Chase/Scatter    (respawns)
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum GhostMode {
    Chase,
    Scatter,
    Frightened,
    Eaten,
}

// ─── Ghost ──────────────────────────────────────────────────────────────────

/// A ghost entity with its type, position, and behavioral state.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ghost {
    pub ghost_type: GhostType,
    pub position: Position,
    pub direction: Direction,
    pub mode: GhostMode,
}

impl Ghost {
    /// Create a ghost at a given starting position.
    ///
    /// # Ownership note
    /// `ghost_type` is `Copy`, so it's passed by value (cheap bitwise copy).
    /// `position` is *moved* into the struct — the caller gives up ownership.
    /// After calling `Ghost::new(GhostType::Blinky, Position::new(14.0, 11.0))`,
    /// the caller can no longer use that `Position` value (it was moved).
    pub fn new(ghost_type: GhostType, position: Position) -> Self {
        Ghost {
            ghost_type,
            position,
            direction: Direction::Up,
            mode: GhostMode::Scatter,
        }
    }

    /// Create all four ghosts at their classic starting positions.
    ///
    /// # Ownership note
    /// Returns a `Vec<Ghost>` — an owned, heap-allocated vector.
    /// The caller takes ownership of the entire vector and all ghosts in it.
    pub fn create_all() -> Vec<Ghost> {
        vec![
            Ghost::new(GhostType::Blinky, Position::new(14.0, 11.0)),
            Ghost::new(GhostType::Pinky, Position::new(12.0, 14.0)),
            Ghost::new(GhostType::Inky, Position::new(14.0, 14.0)),
            Ghost::new(GhostType::Clyde, Position::new(16.0, 14.0)),
        ]
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pacman_starts_with_correct_defaults() {
        let pac = PacMan::new();
        assert_eq!(pac.lives, 3);
        assert_eq!(pac.score, 0);
        assert_eq!(pac.direction, Direction::Left);
    }

    #[test]
    fn pacman_starting_position() {
        let pac = PacMan::new();
        let (col, row) = pac.position.to_grid();
        assert_eq!(col, 14);
        assert_eq!(row, 23);
    }

    #[test]
    fn create_all_ghosts_returns_four() {
        let ghosts = Ghost::create_all();
        assert_eq!(ghosts.len(), 4);
    }

    #[test]
    fn ghosts_have_correct_types() {
        let ghosts = Ghost::create_all();
        assert_eq!(ghosts[0].ghost_type, GhostType::Blinky);
        assert_eq!(ghosts[1].ghost_type, GhostType::Pinky);
        assert_eq!(ghosts[2].ghost_type, GhostType::Inky);
        assert_eq!(ghosts[3].ghost_type, GhostType::Clyde);
    }

    #[test]
    fn ghosts_start_in_scatter_mode() {
        let ghosts = Ghost::create_all();
        for ghost in &ghosts {
            assert_eq!(ghost.mode, GhostMode::Scatter);
        }
    }

    #[test]
    fn position_to_grid_rounds_correctly() {
        let pos = Position::new(13.7, 22.3);
        let (col, row) = pos.to_grid();
        assert_eq!(col, 14);
        assert_eq!(row, 22);
    }
}
