{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
        lib = pkgs.lib;
      in rec {
        packages = rec {
          reviveit = pkgs.callPackage ./backend.nix {};
          default = reviveit;
        };

        devShells.default = pkgs.mkShell (let
          pkg = packages.reviveit;
        in {
          inherit (pkg) buildInputs;
          nativeBuildInputs = pkg.nativeBuildInputs ++ (with pkgs; [rust-analyzer rustfmt nodejs diesel-cli]);
        });

        formatter = pkgs.alejandra;
      }
    );
}
