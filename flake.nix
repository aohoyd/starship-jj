{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-root.url = "github:srid/flake-root";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; }{
      imports = [
        inputs.flake-root.flakeModule
      ];
      systems = [ "x86_64-linux" "x86_64-darwin" ];
      perSystem = { config, self', pkgs, lib, system, ... }:
        let          
          runtimeDeps = with pkgs; [ openssl ];
          buildDeps = with pkgs; [ pkg-config rust-analyzer ];
          devDeps = with pkgs; [];

          # Read some attributes from the root dir Cargo.toml in case values get inherited from workdspace
          rootCargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          rootPackage = rootCargoToml.workspace.package or rootCargoToml.package;
          longDescription = if rootPackage ? readme then builtins.readFile (./. + ("/" + rootPackage.readme)) else null;
          homepage = rootPackage.homePage or rootPackage.repository or null;
          license = rootPackage.license or null;

          rustPackage = bin-dir: features:
            let
              cargoToml = builtins.fromTOML (builtins.readFile (bin-dir  + "/Cargo.toml"));
              name = cargoToml.package.name;
              version = cargoToml.package.version;
              descriptin = cargoToml.package.description or null;
            in
            (pkgs.makeRustPlatform {
              cargo = pkgs.rust-bin.stable.latest.minimal;
              rustc = pkgs.rust-bin.stable.latest.minimal;
            }).buildRustPackage{
              inherit name version;
              src = pkgs.lib.cleanSource ./.;
              cargoLock.lockFile = ./Cargo.lock;
              buildFeatures = features;
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps;
              cargoBuildFlags = ["-p" cargoToml.package.name];
              meta = pkgs.lib.attrsets.filterAttrs (k: v: v != null) {
                inherit homepage license descriptin longDescription;
              };
            };
          mkDevShell = rustc:
            pkgs.mkShell {
              inputsFrom = [config.flake-root.devShell];
              shellHook  = ''
                export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
                export CARGO_TARGET_DIR="$FLAKE_ROOT/target/nixos"
              '';
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps ++ devDeps ++ [rustc];
            };
        in {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [(import inputs.rust-overlay)];
          };

          packages.starship-jj = (rustPackage ./. []);
          packages.default = self'.packages.starship-jj;
          devShells.default = self'.devShells.stable;

          devShells.nightly = (mkDevShell (pkgs.rust-bin.selectLatestNightlyWith
            (toolchain: toolchain.default)));
          devShells.stable = (mkDevShell pkgs.rust-bin.stable.latest.default);
        };
    };
}
