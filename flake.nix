{
  description = "A basic rust devshell";

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
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        
        rustToolchain =
          pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
          };

        nativeBuildInputs = with pkgs; [ rustToolchain cargo-watch pkg-config ];

      in {

        devShells.default = pkgs.mkShell {
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          inherit nativeBuildInputs;
        };
      });
}
