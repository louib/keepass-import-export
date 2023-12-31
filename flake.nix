{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }: (
    flake-utils.lib.eachDefaultSystem (
      system: (
        let
          projectName = "keepass-import-export";
          pkgs = import nixpkgs {
            inherit system;
          };

          cargoPackages = with pkgs; [
            cargo
            rustc
            rustfmt
          ];
        in {
          devShells = {
            default = pkgs.mkShell {
              buildInputs = cargoPackages;

              shellHook = ''
                export RUSTFLAGS='-C target-cpu=native'
              '';
            };
          };
          packages = {
            default = pkgs.rustPlatform.buildRustPackage rec {
              pname = projectName;
              version = "main";

              src = ./.;

              cargoLock = {
                lockFile = ./Cargo.lock;
              };

              meta = with pkgs.lib; {
                description = "CLI tools to export to and import from KDBX (keepass) databases";
                homepage = "https://github.com/louib/${projectName}";
                license = licenses.mit;
                # maintainers = [];
              };
            };
          };
        }
      )
    )
  );
}
