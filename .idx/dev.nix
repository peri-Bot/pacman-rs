# To learn more about how to use Nix to configure your environment
# see: https://firebase.google.com/docs/studio/customize-workspace
{ pkgs, ... }: {
  # Which nixpkgs channel to use.
  channel = "unstable"; # or "unstable"
  # Use https://search.nixos.org/packages to find packages
  packages = [
    pkgs.eza
    pkgs.gh
    pkgs.newman
    pkgs.xclip
    pkgs.rustc
    pkgs.cargo
    pkgs.starship
    pkgs.dwt1-shell-color-scripts
    pkgs.fastfetch
    pkgs.glibc
    pkgs.gcc
    pkgs.fzf
    pkgs.luajitPackages.luarocks_bootstrap
    pkgs.lua
    pkgs.neovim
    pkgs.zig
    pkgs.rustup
    pkgs.go
    pkgs.python311
    pkgs.python311.pkgs.pip
    pkgs.zsh
    pkgs.openssh
    pkgs.bun
    # pkgs.nodePackages.nodemon
		pkgs.gopls
  ];
  # Sets environment variables in the workspace
  env = {		ZSH_DISABLE_COMPFIX = "true";
		NIXPKGS_ALLOW_UNFREE = "1";
  };
  idx = {
    # Search for the extensions you want on https://open-vsx.org/ and use "publisher.id"
    extensions = [
      # "vscodevim.vim" 
    ];
    # Enable previews
    previews = {
      enable = true;
      previews = {
        web = {
          # Example: run "npm run dev" with PORT set to IDX's defined port for previews,
          # and show it in IDX's web preview panel
          command = [
            "bun"
            "run"
            "dev"
            "--"
            "--port"
            "$PORT"
            "--host"
            "0.0.0.0"
          ];
          manager = "web";
          env = {
            # Environment variables to set for your server
            PORT = "$PORT";
          };
        };
      };
    };
    # Workspace lifecycle hooks
    workspace = {
      # Runs when a workspace is first created
      onCreate = {
        # Example: install JS dependencies from NPM
        # npm-install = "npm install";
        # Open editors for the following files by default, if they exist:
        default.openFiles = [ ".idx/dev.nix" "README.md" ];
      };
      # Runs when the workspace is (re)started
      onStart = {
        # Example: start a background task to watch and re-build backend code
        # watch-backend = "npm run watch-backend";
        install-omz = "curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh  | sh";
      };
    };
  };
}