{
  description = "pacman-rs — Pacman game with Vue 3 frontend and Rust+WASM backend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # ── Rust toolchains ──────────────────────────────────────────
        # Native toolchain (for checks, tests, dev shell)
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # Crane lib using our toolchain
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # ── Source filtering ─────────────────────────────────────────
        # Only include Rust source files for crane builds (faster caching)
        gameDir = ./game;
        gameSrc = pkgs.lib.cleanSourceWith {
          src = gameDir;
          filter = path: type:
            (craneLib.filterCargoSources path type);
        };

        # Common args for all crane derivations
        commonArgs = {
          src = gameSrc;
          pname = "pacman-game";
          version = "0.1.0";
        };

        # Build just the cargo dependencies for caching
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # ── Packages ─────────────────────────────────────────────────

        # Step 1: Build Rust → WASM via crane (handles vendoring automatically)
        wasmRaw = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;

          # Cross-compile to WASM
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";

          # Don't try to run tests or install bins (it's a cdylib)
          doCheck = false;
          doInstallCargoArtifacts = false;

          installPhaseCommand = ''
            mkdir -p $out
            cp target/wasm32-unknown-unknown/release/pacman_game.wasm $out/ || true
            cp target/wasm32-unknown-unknown/release/*.wasm $out/ || true
          '';
        });

        # Step 2: Run wasm-bindgen + wasm-opt on the raw .wasm
        wasmPkg = pkgs.stdenv.mkDerivation {
          pname = "pacman-game-wasm";
          version = "0.1.0";
          src = wasmRaw;
          nativeBuildInputs = [ pkgs.wasm-bindgen-cli pkgs.binaryen ];

          buildPhase = ''
            # Generate JS bindings
            wasm-bindgen \
              --target web \
              --out-dir $out \
              pacman_game.wasm

            # Optimize for size
            wasm-opt -Os -o $out/pacman_game_bg.wasm $out/pacman_game_bg.wasm
          '';

          installPhase = "true";
        };

      in
      {
        # ── Development Shell ──────────────────────────────────────
        devShells.default = craneLib.devShell {
          # Extra tools beyond what crane provides
          packages = with pkgs; [
            wasm-pack
            wasm-bindgen-cli
            binaryen
            bun
            nodejs_22
            pkg-config
            openssl
            just
          ];

          shellHook = ''
            echo ""
            echo "🕹️  pacman-rs dev shell activated"
            echo "──────────────────────────────────"
            echo "  rustc    : $(rustc --version)"
            echo "  cargo    : $(cargo --version)"
            echo "  wasm-pack: $(wasm-pack --version)"
            echo "  bun      : $(bun --version)"
            echo "  node     : $(node --version)"
            echo "──────────────────────────────────"
            echo ""
            echo "Commands:"
            echo "  bun install && bun run dev     # Dev server"
            echo "  wasm-pack build game/          # Build WASM"
            echo "  nix build                      # Production WASM build"
            echo "  nix flake check                # Run all CI checks"
            echo ""
          '';
        };

        # ── Packages ────────────────────────────────────────────────
        # `nix build`       → WASM package (pure, sandboxed)
        # Frontend build runs via `nix develop --command bun run build`
        # because bun needs network access to install JS dependencies,
        # which isn't allowed in Nix sandboxed builds.
        packages = {
          default = wasmPkg;
          wasm = wasmPkg;
        };

        # ── Checks (run via `nix flake check`) ──────────────────────
        checks = {
          # Verify formatting
          fmt = craneLib.cargoFmt commonArgs;

          # Lint with clippy
          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "-- -D warnings";
          });

          # Run tests
          tests = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
          });

          # Verify WASM build works
          wasm-build = wasmPkg;
        };
      }
    );
}
