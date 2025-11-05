{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flakeUtils.url = "github:numtide/flake-utils";
  };

  outputs = { flakeUtils, nixpkgs, ... }:
    flakeUtils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.callPackage ./shell.nix { };
        packages.default = pkgs.callPackage ./default.nix { };
      }
    );
}
