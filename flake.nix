{
  description = "Multi-platform mod manager for the game barony";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      rec {
        commonDependencies = with pkgs; [
          cmake
          freetype
          openssl
          # TODO: On NixOS, this will cause trouble to build
          # (workaround: sudo ln -s `which file` /usr/bin/file)
          file
          expat
          libxkbcommon
        ];

        graphicalDependencies = with pkgs; [
          vulkan-loader
          vulkan-tools
          wayland
          wayland-protocols
          swiftshader
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
        ];

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Project
            rustc
            cargo

            # Development
            clippy
            rustfmt
            rust-analyzer
          ] ++ commonDependencies ++ graphicalDependencies;

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath graphicalDependencies}";
          '';
        };
      }
    );
}
