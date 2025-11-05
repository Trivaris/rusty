{ pkgs ? import <nixpkgs> { } }:
let
  inherit (pkgs)
    mkShell
    rust
    cargo
    rustc
    rustfmt
    clippy
    rust-analyzer
    ;
in
mkShell {
  RUST_SOURCE_PATH = rust.packages.stable.rustPlatform.rustLibSrc;

  packages = [
    cargo
    rustc
    rustfmt
    clippy
    rust-analyzer
  ];
}
