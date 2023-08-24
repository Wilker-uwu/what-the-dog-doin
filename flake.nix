{
  description = "What the dog doin?";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.flake-compat = {
    url = "github:edolstra/flake-compat";
    flake = false;
  };
  inputs.fenix = {
    url = github:nix-community/fenix;
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, flake-compat, fenix }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      overlays = [ fenix.overlays.default ];
    in
    {
      packages = forAllSystems(system:
      {
        default = let
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          toolchain = fenix.packages.${system}.minimal.toolchain;
          manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        in (pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        }).buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          cargoLock.lockFile = ./Cargo.lock;
          src = ./.;

          nativeBuildInputs = with pkgs; [
            pkg-config
            alsa-lib
          ];

          PKG_CONFIG_PATH = "${pkgs.alsa-lib.dev}/lib/pkgconfig"; 
        };
      });

      devShells = forAllSystems(system: let
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        default = pkgs.mkShell {
          buildInputs =
            with pkgs;
            with fenix.packages.${system};
            [
              pkg-config
              alsa-lib
              rust-analyzer-nightly
            ];
          shellHook = ''
            export PGK_CONFIG_PATH="${pkgs.alsa-lib.dev}/lib/pkgconfig";
          '';
        };
      });
    };
}
