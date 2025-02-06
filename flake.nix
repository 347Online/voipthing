{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      utils,
      ...
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell =
          with pkgs;
          mkShell {
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

            buildInputs = lib.optionals stdenv.isLinux [
              alsa-lib
              nasm
              opencv
              clang
            ];

            nativeBuildInputs = lib.optionals stdenv.isLinux [
              pkg-config
              cmake
            ];
          };
      }
    );
}
