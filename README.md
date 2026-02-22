# 🕹️ pacman-rs

A modern Pac-Man game with a **Rust + WebAssembly** backend and a **Vue 3** frontend, featuring a 3D arcade cabinet experience powered by Three.js.

All game logic — movement, ghost AI, collision detection, scoring — runs in Rust compiled to WASM. The browser handles only input and rendering, resulting in near-native performance at 60 fps.

---

## ✨ Features

- **Classic Pac-Man gameplay** — 28×31 maze, dots, power pellets, tunnel wrapping, and lives
- **Four unique ghost AIs** — Blinky (chaser), Pinky (ambusher), Inky (flanker), and Clyde (random/shy), each with distinct targeting logic
- **Ghost behavior modes** — Scatter, Chase, Frightened, and Eaten with timed phase transitions
- **PvP mode** — local 1v1 where Player 2 controls the ghosts (Blinky) via WASD
- **3D arcade cabinet** — an interactive Three.js scene with a GLTF arcade machine model, dynamic lighting, neon flickers, and a GSAP-animated camera zoom into the screen
- **Retro loading screen** — animated ghost parade, Pac-Man chomp animation, and a smooth progress bar
- **CRT-style game menu** — scanline effects, glowing text, and arcade-inspired UI

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────┐
│                    Browser (JS)                     │
│  ┌──────────┐  ┌───────────────┐  ┌──────────────┐ │
│  │ Vue 3    │  │ Three.js /    │  │ Canvas 2D    │ │
│  │ Router   │  │ TresJS Scene  │  │ Game Render  │ │
│  └────┬─────┘  └───────────────┘  └──────┬───────┘ │
│       │            input ↓    state ↑     │         │
│  ┌────┴───────────────────────────────────┴───────┐ │
│  │         wasm-bindgen  (JS ↔ Rust bridge)       │ │
│  └────────────────────┬───────────────────────────┘ │
└───────────────────────┼─────────────────────────────┘
                        │
┌───────────────────────┼─────────────────────────────┐
│              Rust / WASM  (game crate)              │
│  ┌──────────┐ ┌──────────┐ ┌────────────┐           │ 
│  │ state.rs │ │  maze.rs │ │entities.rs │           │
│  └──────────┘ └──────────┘ └────────────┘           │
│                                                     │
│                                                     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

**Key design decisions:**

- **Opaque wrapper pattern** — `GameState` is exposed to JS as an opaque `wasm-bindgen` handle. The internal `GameStateInner` (which contains `Vec`s and nested structs) is serialised via `serde-wasm-bindgen` only when the frontend needs a snapshot.
- **All logic in Rust** — JS never mutates game state directly; it sends direction inputs and calls `tick()`.
- **WASM size optimised** — release builds use `opt-level = "s"`, LTO, and `wasm-opt -Os`.

## 📂 Project Structure

```
pacman-rs/
├── game/                    # Rust crate → compiled to WASM
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs           # WASM entry point & wasm-bindgen exports
│   │   ├── state.rs         # GameState / GameStateInner, tick loop, timers,Pac-Man movement, wall collision, dot/pellet eating,Per-ghost targeting & movement AI
│   │   ├── maze.rs          # 28×31 grid, cell types, walkability
│   │   ├── entities.rs      # PacMan, Ghost, Position, Direction enums
│   └── pkg/                 # wasm-pack build output (git-ignored)
│
├── src/                     # Vue 3 frontend
│   ├── main.js              # App bootstrap
│   ├── App.vue              # Root component
│   ├── router/index.js      # Routes: / → /arcade-machine → /play
│   ├── stores/
│   │   └── arcadeModel.js   # Shared GLTF model store (loaded once)
│   ├── views/
│   │   ├── LandingView.vue         # Loading screen (GLTF preload + progress bar)
│   │   ├── ArcadeMachineView.vue   # 3D arcade cabinet scene + menu overlay
│   │   └── GameView.vue            # WASM init + GameCanvas mount
│   └── components/
│       ├── LoadingScreen.vue   # Animated retro loading UI
│       ├── ArcadeScene.vue     # Three.js scene (cabinet, room, lighting, zoom)
│       ├── arcadeRoom.js       # Procedural arcade room geometry & neon lights
│       ├── GameMenu.vue        # Classic/PvP/Credits menu with CRT effects
│       └── GameCanvas.vue      # Canvas 2D renderer + game loop (60 fps)
│
├── public/
│   └── pacman_arcade/       # GLTF arcade cabinet model + textures
│
├── flake.nix                # Nix flake: dev shell, WASM build, CI checks
├── flake.lock
├── package.json             # Frontend dependencies (Bun)
├── bun.lock
├── vite.config.js           # Vite + WASM + TailwindCSS 4 + TresJS config
├── index.html
└── .github/workflows/ci.yml # GitHub Actions CI pipeline
```

## 🚀 Getting Started

### Prerequisites

- [Nix](https://nixos.org/download/) (with flakes enabled) — **recommended**, provides everything automatically
- Alternatively, install manually:
  - [Rust](https://rustup.rs/) (stable) with the `wasm32-unknown-unknown` target
  - [wasm-pack](https://rustwasm.github.io/wasm-pack/)
  - [Bun](https://bun.sh/) (or Node.js 22+)
  - [wasm-bindgen-cli](https://crates.io/crates/wasm-bindgen-cli)

### Setup with Nix (recommended)

```bash
# Enter the dev shell — installs Rust, wasm-pack, Bun, Node, etc.
nix develop

# Install frontend dependencies
bun install

# Build the WASM package
wasm-pack build game/ --target web

# Start the dev server
bun run dev
```

### Setup without Nix

```bash
# Add the WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
cargo install wasm-pack

# Build the WASM package
wasm-pack build game/ --target web

# Install frontend dependencies
bun install

# Start the dev server
bun run dev
```

The app will be available at `http://localhost:5173`.

## 🎮 Controls

| Action              | Player 1 (Pac-Man) | Player 2 (Ghost — PvP only) |
|---------------------|---------------------|-----------------------------|
| Move Up             | `↑` Arrow           | `W`                        |
| Move Down           | `↓` Arrow           | `S`                        |
| Move Left           | `←` Arrow           | `A`                        |
| Move Right          | `→` Arrow           | `D`                        |

**Getting to the game:**

1. The landing page loads the 3D arcade cabinet model
2. Press **Space** to zoom into the arcade screen
3. Choose **Classic** (single-player) or **PvP** (local 1v1)

## 🔧 Development

### Available Scripts

| Command | Description |
|---------|-------------|
| `bun run dev` | Start Vite dev server with HMR |
| `bun run build` | Production build |
| `bun run preview` | Preview production build locally |
| `bun run lint` | Run OxLint + ESLint |
| `bun run format` | Format code with Prettier |

### Rust / WASM Development

```bash
# Run Rust tests natively
cargo test --manifest-path game/Cargo.toml

# Rebuild WASM after Rust changes
wasm-pack build game/ --target web

# Check formatting
cargo fmt --manifest-path game/Cargo.toml --check

# Lint with Clippy
cargo clippy --manifest-path game/Cargo.toml -- -D warnings
```

### Nix Commands

```bash
# Enter dev shell
nix develop

# Build the WASM package (pure, sandboxed)
nix build

# Run all CI checks (fmt, clippy, tests, WASM build)
nix flake check
```

## 🧰 Tech Stack

### Frontend

| Technology | Purpose |
|-----------|---------|
| [Vue 3](https://vuejs.org/) | UI framework (Composition API, `<script setup>`) |
| [Vue Router](https://router.vuejs.org/) | SPA routing |
| [Three.js](https://threejs.org/) + [TresJS](https://tresjs.org/) | 3D arcade cabinet scene |
| [GSAP](https://greensock.com/gsap/) | Camera zoom animation |
| [TailwindCSS 4](https://tailwindcss.com/) | Utility-first CSS |
| [Vite](https://vite.dev/) | Dev server & bundler |
| [vite-plugin-wasm](https://github.com/nicolo-ribaudo/vite-plugin-wasm) | WASM module support |

### Backend (Game Engine)

| Technology | Purpose |
|-----------|---------|
| [Rust](https://www.rust-lang.org/) | Game logic (zero-cost abstractions, memory safety) |
| [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) | Rust ↔ JS interop bridge |
| [serde](https://serde.rs/) + [serde-wasm-bindgen](https://crates.io/crates/serde-wasm-bindgen) | State serialisation to JS objects |
| [wasm-pack](https://rustwasm.github.io/wasm-pack/) | Build tooling for Rust → WASM |

### Build & CI

| Technology | Purpose |
|-----------|---------|
| [Nix Flakes](https://nixos.wiki/wiki/Flakes) | Reproducible dev environment & builds |
| [Crane](https://crane.dev/) | Nix-native Rust/WASM builds with cargo caching |
| [GitHub Actions](https://github.com/features/actions) | CI pipeline (lint, test, build) |
| [Bun](https://bun.sh/) | Fast JS package manager & runtime |

## 🔄 CI Pipeline

The GitHub Actions workflow (`.github/workflows/ci.yml`) runs on every push/PR to `main`:

1. **`nix flake check`** — Rust formatting, Clippy lints, native tests, and WASM build verification (all sandboxed)
2. **`bun install --frozen-lockfile`** — install frontend dependencies
3. **`wasm-pack build game/ --target web`** — build WASM for the frontend
4. **`bun run build`** — full Vite production build

## 📝 License

This project is private. All rights reserved.
