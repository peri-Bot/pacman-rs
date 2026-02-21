// game/src/maze.rs
//
// The Pac-Man maze represented as a 2D grid of cells.
//
// The classic Pac-Man maze is 28 columns × 31 rows.
// Each cell is an enum variant describing what occupies that position.

use serde::{Deserialize, Serialize};

// ─── Cell types ─────────────────────────────────────────────────────────────

/// Every cell in the maze is exactly one of these variants.
///
/// # Derive traits explained
/// - `Clone` + `Copy`: small stack-only enum, can be duplicated cheaply by value
/// - `PartialEq`: enables `==` / `!=` comparisons between cells
/// - `Debug`: allows `println!("{:?}", cell)` for debugging
/// - `Serialize` / `Deserialize`: serde traits so we can send the maze to JS
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CellType {
    Empty,
    Wall,
    Dot,
    PowerPellet,
    GhostHouse,
}

// ─── Maze dimensions ────────────────────────────────────────────────────────

pub const MAZE_WIDTH: usize = 28;
pub const MAZE_HEIGHT: usize = 31;

// ─── Maze struct ────────────────────────────────────────────────────────────

/// The game maze: a 2D grid stored as `Vec<Vec<CellType>>`.
///
/// # Why Vec<Vec<CellType>> instead of a flat Vec?
/// Readability and ease of indexing: `maze.cells[row][col]`.
/// For a 28×31 grid (~868 cells), the performance difference is negligible.
/// A flat array with manual index math would be faster for huge grids,
/// but for Pac-Man's fixed-size maze, clarity wins.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Maze {
    pub cells: Vec<Vec<CellType>>,
    pub width: usize,
    pub height: usize,
}

impl Maze {
    /// Create the classic Pac-Man maze layout.
    ///
    /// # Ownership note
    /// This function returns an *owned* `Maze`. The caller takes full ownership.
    /// The `Vec`s are heap-allocated; when the `Maze` is dropped, Rust
    /// automatically frees them (no garbage collector needed — this is RAII).
    pub fn new() -> Self {
        // Legend:
        //   W = Wall
        //   . = Dot
        //   o = Power Pellet
        //   G = Ghost House
        //   E = Empty (tunnels, ghost house entry)
        //
        // This is a simplified classic layout. Each string is one row (28 chars).
        let layout: Vec<&str> = vec![
            "WWWWWWWWWWWWWWWWWWWWWWWWWWWW",  // 0
            "W............WW............W",  // 1
            "W.WWWW.WWWWW.WW.WWWWW.WWWWW",   // 2  (was: "W.WWWW.WWWWW.WW.WWWWW.WWWW.")
            "WoWWWW.WWWWW.WW.WWWWW.WWWWoW",  // 3  (was: "WoWWWW.WWWWW.WW.WWWWW.WWWWo.")
            "W.WWWW.WWWWW.WW.WWWWW.WWWWW",   // 4  (was: same pattern)
            "W..........................W",  // 5
            "W.WWWW.WW.WWWWWWWW.WW.WWWWW",   // 6  (was: "W.WWWW.WW.WWWWWWWW.WW.WWWW.")
            "W.WWWW.WW.WWWWWWWW.WW.WWWWW",   // 7
            "W......WW....WW....WW......W",  // 8
            "WWWWWW.WWWWW.WW.WWWWW.WWWWWW",  // 9  (was: "WWWWWW.WWWWWEWWEEWWWWW.WWWWWW")
            "EEEEWW.WWWWW.WW.WWWWW.WWEEEEE", // 10 — fixed to 28 below
            "EEEEWW.WW..........WW.WWEEEEE", // 11
            "EEEEWW.WW.WWWGGWWW.WW.WWEEEEE", // 12
            "WWWWWW.WW.WEGGGGEW.WW.WWWWWW",  // 13
            "EEEEEE....WEGGGGEW....EEEEEE",  // 14  ← tunnel row
            "WWWWWW.WW.WEGGGGEW.WW.WWWWWW",  // 15
            "EEEEWW.WW.WWWWWWWW.WW.WWEEEEE", // 16
            "EEEEWW.WW..........WW.WWEEEEE", // 17
            "EEEEWW.WW.WWWWWWWW.WW.WWEEEEE", // 18
            "WWWWWW.WW.WWWWWWWW.WW.WWWWWW",  // 19
            "W............WW............W",  // 20
            "W.WWWW.WWWWW.WW.WWWWW.WWWWW",   // 21
            "W.WWWW.WWWWW.WW.WWWWW.WWWWW",   // 22
            "Wo..WW................WW..oW",  // 23
            "WWW.WW.WW.WWWWWWWW.WW.WW.WWW",  // 24
            "WWW.WW.WW.WWWWWWWW.WW.WW.WWW",  // 25
            "W......WW....WW....WW......W",  // 26
            "W.WWWWWWWWWW.WW.WWWWWWWWWW.W",  // 27
            "W.WWWWWWWWWW.WW.WWWWWWWWWW.W",  // 28
            "W..........................W",  // 29
            "WWWWWWWWWWWWWWWWWWWWWWWWWWWW",  // 30
        ];

        let cells: Vec<Vec<CellType>> = layout
            .iter()
            .map(|row| {
                let mut row_cells: Vec<CellType> = row
                    .chars()
                    .take(MAZE_WIDTH) // Ensure exactly 28 columns
                    .map(|ch| match ch {
                        'W' => CellType::Wall,
                        '.' => CellType::Dot,
                        'o' => CellType::PowerPellet,
                        'G' => CellType::GhostHouse,
                        _ => CellType::Empty, // 'E' and anything else
                    })
                    .collect();
                // Pad or trim to exactly MAZE_WIDTH
                row_cells.resize(MAZE_WIDTH, CellType::Empty);
                row_cells
            })
            .collect();

        Maze {
            cells,
            width: MAZE_WIDTH,
            height: MAZE_HEIGHT,
        }
    }

    /// Count remaining dots (regular + power pellets) on the maze.
    pub fn dots_remaining(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| **cell == CellType::Dot || **cell == CellType::PowerPellet)
            .count()
    }

    /// Get the cell type at a grid position, or `None` if out of bounds.
    ///
    /// # Borrowing note
    /// Returns `Option<CellType>` — because `CellType` is `Copy`,
    /// we return it by value (a cheap copy), not a reference.
    pub fn get_cell(&self, row: usize, col: usize) -> Option<CellType> {
        self.cells.get(row).and_then(|r| r.get(col)).copied()
    }

    /// Check if a coordinate is walkable by entities (not a wall or ghost house).
    pub fn is_walkable(&self, x: f64, y: f64) -> bool {
        let ix = x.round() as isize;
        let iy = y.round() as isize;

        if ix < 0 || ix >= self.width as isize {
            return true; // tunnels are walkable wrap-arounds
        }

        let cell = self.get_cell(iy as usize, ix as usize);
        !matches!(
            cell,
            Some(CellType::Wall) | Some(CellType::GhostHouse) | None
        )
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maze_has_correct_dimensions() {
        let maze = Maze::new();
        assert_eq!(maze.height, MAZE_HEIGHT);
        assert_eq!(maze.width, MAZE_WIDTH);
        assert_eq!(maze.cells.len(), MAZE_HEIGHT);
        for (i, row) in maze.cells.iter().enumerate() {
            assert_eq!(
                row.len(),
                MAZE_WIDTH,
                "Row {} has {} cells, expected {}",
                i,
                row.len(),
                MAZE_WIDTH
            );
        }
    }

    #[test]
    fn maze_corners_are_walls() {
        let maze = Maze::new();
        assert_eq!(maze.get_cell(0, 0), Some(CellType::Wall));
        assert_eq!(maze.get_cell(0, MAZE_WIDTH - 1), Some(CellType::Wall));
        assert_eq!(maze.get_cell(MAZE_HEIGHT - 1, 0), Some(CellType::Wall));
        assert_eq!(
            maze.get_cell(MAZE_HEIGHT - 1, MAZE_WIDTH - 1),
            Some(CellType::Wall)
        );
    }

    #[test]
    fn maze_has_dots() {
        let maze = Maze::new();
        assert!(maze.dots_remaining() > 0, "Maze should contain dots");
    }

    #[test]
    fn maze_has_power_pellets() {
        let maze = Maze::new();
        let pellet_count = maze
            .cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| **cell == CellType::PowerPellet)
            .count();
        assert_eq!(pellet_count, 4, "Classic maze should have 4 power pellets");
    }

    #[test]
    fn maze_has_ghost_house() {
        let maze = Maze::new();
        let ghost_cells = maze
            .cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| **cell == CellType::GhostHouse)
            .count();
        assert!(ghost_cells > 0, "Maze should have a ghost house");
    }

    #[test]
    fn get_cell_out_of_bounds_returns_none() {
        let maze = Maze::new();
        assert_eq!(maze.get_cell(100, 100), None);
    }
}
