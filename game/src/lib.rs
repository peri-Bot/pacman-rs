// game/src/lib.rs
//
// Entry point for the Pac-Man WASM module.
// All game logic lives in Rust; JavaScript only handles input and rendering.

// Allow dead code — Phase 2 scaffolding methods will be used by game logic in later phases.
#![allow(dead_code)]

// Modules — each file becomes a module
mod entities;
mod maze;
mod state;

// Re-export the GameState so JS can access it directly via `import { GameState } from '...'`
pub use state::GameState;

use wasm_bindgen::prelude::*;

// ─── Phase 1: WASM bridge proof-of-concept ──────────────────────────────────

/// A simple function to verify JS↔Rust communication works.
///
/// # Ownership note
/// `name: &str` is a *borrowed reference* — we can read it but don't own it.
/// The JS caller retains ownership of the string memory.
/// The returned `String` is an *owned* value that wasm-bindgen serializes
/// across the WASM boundary and then frees on the Rust side.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello from Rust, {}! 🦀", name)
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_returns_expected_message() {
        assert_eq!(greet("Pac-Man"), "Hello from Rust, Pac-Man! 🦀");
    }

    #[test]
    fn greet_handles_empty_name() {
        assert_eq!(greet(""), "Hello from Rust, ! 🦀");
    }
}
