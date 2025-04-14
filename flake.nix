{
  description = "Starship-jj shows jujutsu-vcs status for the starship prompt";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    systems.url = "github:nix-systems/default";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
  };
  outputs =
    {
      self,
      flake-utils,
      fenix,
      nixpkgs,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        rustBuildToolchain = fenix.packages.${system}.stable.minimalToolchain;
        rustDevToolchain = fenix.packages.${system}.stable.toolchain;
        rootCargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        rootPackage = rootCargoToml.workspace.package or rootCargoToml.package or null;
        longDescription =
          if rootPackage ? readme then builtins.readFile (./. + ("/" + rootPackage.readme)) else null;
        homepage = rootPackage.homePage or rootPackage.repository or null;
        license = rootPackage.license or null;

        rustPackage =
          bin-dir: features:
          with builtins;
          let
            cargoToml = fromTOML (readFile (bin-dir + "/Cargo.toml"));
            pname = cargoToml.package.name;
            inherit (cargoToml.package) version;
            description = cargoToml.package.description or null;
          in
          with pkgs;
          (makeRustPlatform {
            cargo = rustBuildToolchain;
            rustc = rustBuildToolchain;
          }).buildRustPackage
            {
              inherit pname version;
              src = lib.cleanSource ./.;
              cargoLock.lockFile = ./Cargo.lock;
              buildFeatures = features;
              buildInputs = [ openssl ];
              nativeBuildInputs = [ pkg-config ];
              cargoBuildFlags = [
                "-p"
                cargoToml.package.name
              ];
              meta = lib.attrsets.filterAttrs (k: v: v != null) {
                inherit
                  homepage
                  license
                  description
                  longDescription
                  ;
              };
            };
      in
      {
        packages.default = self.packages."${system}".starship-jj;
        packages.starship-jj = rustPackage ./. [ ];

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rust-analyzer
            rustDevToolchain
          ];
          inputsFrom = [ self.packages."${system}".default ];
        };
      }
    )
    // {
      overlays.default = final: prev: { inherit (self.packages."${prev.system}") starship-jj; };
    };
}
