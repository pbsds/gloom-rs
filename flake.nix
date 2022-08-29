{
  description = "Gloom, but rustier";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-22.05";

    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Needed to run OpenGL software through Nix on non-NixOS machines.
    nixGL = {
      url = "github:guibou/nixGL";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, nixGL, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        buildInputs = with pkgs; [ fontconfig freetype ];

        nativeBuildInputs = with pkgs; [ cmake pkg-config ];

        LD_LIBRARY_PATH = with pkgs;
          lib.makeLibraryPath ([
            libGL
            libxkbcommon
            wayland
          ] ++ (with xorg; [
            libX11
            libXcursor
            libXi
            libXrandr
          ]));

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

      in {
        packages = {
          default = self.packages.${system}.gloom-rs;
          gloom-rs = pkgs.rustPlatform.buildRustPackage {
            pname = "gloom-rs";
            inherit (cargoToml.package) version;
            src = ./.;

            cargoSha256 = "ozFzkPYScIL/REZYpiG+ex/zNIKiKFohFx/jiB6tbzs=";

            inherit buildInputs;
            nativeBuildInputs = nativeBuildInputs ++ [ pkgs.makeWrapper ];

            postFixup = ''
              wrapProgram $out/bin/gloom-rs \
                --prefix LD_LIBRARY_PATH : ${LD_LIBRARY_PATH}
            '';

            meta = with pkgs.lib; {
              description = "Gloom, but rustier";
              longDescription = ''
                Gloom-rs is the setup for exercises in TDT4195 - Visual Computing Fundamentals 2022.
                It's built using Rust and OpenGL.
              '';
              homepage = "https://github.com/pbsds/gloom-rs";
              platforms = flake-utils.lib.defaultSystems;
              maintainers = let
                parseAuthor = authorString:
                  let a = splitString "<" authorString;
                  in {
                    name = removeSuffix " " (head a);
                    email = removeSuffix ">" (head (tail a));
                  };
              in map parseAuthor cargoToml.package.authors;
            };
          };
        };

        overlays.default = final: prev: { inherit (self.packages.${system}) gloom-rs; };

        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          nativeBuildInputs = nativeBuildInputs ++ [ pkgs.rustup ];

          inherit LD_LIBRARY_PATH;

          shellHook = ''
            rustup override set stable
          '';
        };

        apps = let
          nixGLWrap = nixGLPkg: nixGLExecutable: {
            type = "app";
            program = toString (pkgs.writeShellScript "gloom-rs.sh"
              "${nixGL.packages.${system}.${nixGLPkg}}/bin/${nixGLExecutable} ${
                self.packages.${system}.default
              }/bin/gloom-rs");
          };
          nixGLWrapWithSameName = nixGLPkg:
            nixGLWrap nixGLPkg "${nixGL.packages.${system}.${nixGLPkg}.name}";
        in {
          default = self.apps.${system}.nixosCompatible;

          # This runs the app raw. It will not work on non-nixos machines.
          # For more info, see: https://nixos.wiki/wiki/Nixpkgs_with_OpenGL_on_non-NixOS
          nixosCompatible = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/gloom-rs";
          };

          # These requires you to run in impure mode, and some uses unfree drivers.
          # Example:
          #   $ NIXPKGS_ALLOW_UNFREE=1 nix run .#nvidia --impure
          #
          # Also note that the Auto version might not work properly for some systems.
          auto = nixGLWrap "nixGLDefault" "nixGL";
          nvidia = nixGLWrapWithSameName "nixGLNvidia";
          nvidiaBumblebee = nixGLWrapWithSameName "nixGLNvidiaBumblebee";
          intel = nixGLWrapWithSameName "nixGLIntel";
        };
      });
}
