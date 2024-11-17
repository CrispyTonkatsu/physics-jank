{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        libPath = with pkgs; 
          [ 
            libffi
            wayland-protocols
            wayland
            libGL
            xorg.libxcb
            glfw

            pkg-config

            xorg.libX11
            xorg.libX11.dev
            xorg.libXft
            xorg.libXrandr
            xorg.libXinerama
            xorg.libXcursor
            xorg.libXi

            libxkbcommon

            libglvnd
          ]; 
      in
        {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          nativeBuildInputs = [
            wayland-protocols
            libxkbcommon
            wayland
          ];
          buildInputs = [ 
            cargo 
            rustc 
            rustfmt 
            pre-commit 
            rustPackages.clippy 
            cmake
          ]++ libPath;
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = lib.makeLibraryPath libPath;
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        };
      }
    );
}
