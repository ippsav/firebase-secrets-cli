{
  description = "A powerful command-line tool built with Rust, allowing users to easily set multiple secrets at once from a file containing key-value pairs. This eliminates the need for manual, time-consuming configuration of individual secrets, streamlining the process and simplifying the management of sensitive data in Firebase projects.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    gitignore = { url = "github:hercules-ci/gitignore"; flake = false; };
  };

  outputs = { self, nixpkgs, flake-utils, ...}:
    let
        system = "x86_64-linux";
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib;
        inherit (nixpkgs.lib.importTOML (self + "/Cargo.toml")) package;
    in
    {
      packages.${system} = {
        ${package.name} = pkgs.rustPlatform.buildRustPackage {
          pname = package.name;
          src = ./.;
          version = package.version;
          cargoHash = "sha256-BtWH1y9h3RLGH+koMIagxj4vMk7Gq0qZSfen2uANztU=";

          nativeBuildInputs = [
            pkgs.makeWrapper
          ];

          postInstall = ''
            wrapProgram $out/bin/${package.name} \
              --prefix PATH : ${lib.makeBinPath [
                pkgs.nodePackages.firebase-tools
              ]}
          '';

        };
      };

      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          pkgs.nodePackages.firebase-tools
          pkgs.rustc
          pkgs.cargo
          pkgs.rust-analyzer
          pkgs.neovim
        ];
        shellHook = ''
          export PATH="$PATH:$HOME/.cargo/bin"
        '';
      };
  };
}
