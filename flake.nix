{
  description = "Rust dev shell for building project with fontconfig and OpenGL support on nixos";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        nativeBuildInputs = with pkgs; [
          git
	  pkg-config
          fontconfig
          freetype
          rustup
          cmake
          gnumake
	  gcc
          cargo
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libX11
          xorg.libxcb
          libxkbcommon
          libGL
          libGLU
          mesa
          wayland
          libglibutil
        ];

        fontconfigPcPath = "${pkgs.fontconfig.dev}/lib/pkgconfig";
      in {
        devShells.default = pkgs.mkShell {
          name = "rust-fontconfig-shell";
          packages = nativeBuildInputs;

          shellHook = ''
            export PKG_CONFIG_PATH=${fontconfigPcPath}:$PKG_CONFIG_PATH
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath nativeBuildInputs}:$LD_LIBRARY_PATH
            echo "PKG_CONFIG_PATH set to: ${fontconfigPcPath}"
            echo "LD_LIBRARY_PATH set for runtime OpenGL/GUI deps"
          '';
        };
      });
}

