{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, naersk, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        libPath = with pkgs;
          lib.makeLibraryPath [
            libGL
            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ];

        nativeBuildInputs = with pkgs; [
          rustToolchain
          cargo-watch
          pkg-config
          xorg.libxcb
          hyperfine
        ];

        naersk-lib = pkgs.callPackage naersk { };
      in {
        # nothing in this package, just for CI.
        defaultPackage = naersk-lib.buildPackage {
          inherit nativeBuildInputs;
          root = ./.;
        };

        devShells.default = pkgs.mkShell {
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          LD_LIBRARY_PATH = libPath;

          inherit nativeBuildInputs;
        };
      });
}
