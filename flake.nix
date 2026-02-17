{
  description = "pacman-rs — Pacman game with Vue 3 frontend and Rust+WASM backend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain: stable with wasm32 target
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        # ──────────────────────────────────────────────
        # Development Shell
        # ──────────────────────────────────────────────
        devShells.default = pkgs.mkShell {
          name = "pacman-rs-dev";

          buildInputs = with pkgs; [
            # ── Rust ──
            rustToolchain
            wasm-pack
            wasm-bindgen-cli
            binaryen          # wasm-opt

            # ── Node / Frontend ──
            bun
            nodejs_22

            # ── Common native deps for Rust crates ──
            pkg-config
            openssl

            # ── Utilities ──
            just              # task runner
          ];

          shellHook = ''
            echo ""
            echo "🕹️  pacman-rs dev shell activated"
            echo "──────────────────────────────────"
            echo "  rustc   : $(rustc --version)"
            echo "  cargo   : $(cargo --version)"
            echo "  wasm-pack: $(wasm-pack --version)"
            echo "  bun     : $(bun --version)"
            echo "  node    : $(node --version)"
            echo "──────────────────────────────────"
            echo ""
            echo "Quick start:"
            echo "  bun install && bun run dev   # Vue frontend"
            echo "  wasm-pack build game/        # Build Rust→WASM (once game/ crate exists)"
            echo ""
          '';
        };

        # ──────────────────────────────────────────────
        # Production Package (Vue frontend build)
        # ──────────────────────────────────────────────
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "pacman-rs";
          version = "0.1.0";

          src = pkgs.lib.cleanSource ./.;

          nativeBuildInputs = with pkgs; [
            bun
            nodejs_22
          ];

          # bun install needs a writable home
          buildPhase = ''
            export HOME=$(mktemp -d)
            bun install --frozen-lockfile
            bun run build
          '';

          installPhase = ''
            mkdir -p $out
            cp -r dist/* $out/
          '';
        };
      }
    );
}
