{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
      in
      {
        devShells.default = pkgs.mkShell rec {
          packages = [
            # pkgs.clippy
            (pkgs.rust-bin.stable.latest.default.override {
                extensions = [
                  "rust-src" # for rust-analyzer
                  "rust-analyzer"
                  "clippy"
                ];
              }
            )
          ];

          nativeBuildInputs = [
            pkgs.pkg-config
          ];

          buildInputs = [
            pkgs.openssl
            pkgs.openblas
            pkgs.lapack
            pkgs.gccNGPackages_15.gfortran-unwrapped
            pkgs.gccNGPackages_15.libgfortran
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
