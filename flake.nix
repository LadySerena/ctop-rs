{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.systems.follows = "systems";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { flake-utils, nixpkgs, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        nativeBuildInputs = with pkgs; [
          (rust-bin.stable.latest.default.override {
            extensions = [ "rust-analyzer" "rust-src" ];
          })
          procps
          clang
          libllvm
          libclang
          libclang.lib
        ];
        buildInputs = with pkgs; [ pkg-config ];
      in with pkgs; {
        devShells.default = mkShell {
          inherit nativeBuildInputs buildInputs;
          shellHook = ''
            export LIBCLANG_PATH="${pkgs.libclang.lib}/lib";
          '';
        };
      });
}
