{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };
        naersk' = pkgs.callPackage naersk { };
        buildInputs = with pkgs; [
          udev
          alsa-lib
          vulkan-loader

          #X11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr

          #Wayland
          libxkbcommon 
          wayland

          #Web
          trunk
          sass
          dart-sass
          wasm-bindgen-cli
        ];
        nativeBuildInputs = with pkgs; [
          pkg-config
          #libxkbcommon
          (rust-bin.selectLatestNightlyWith
            (toolchain: toolchain.default.override {
              extensions = [ "rust-src" "clippy" ];
              targets = [ "wasm32-unknown-unknown" ];
            }))
        ];
        all_deps = with pkgs; [
          cargo-watch
          cargo-flamegraph
          cargo-expand
          nixpkgs-fmt
          cmake
        ] ++ buildInputs ++ nativeBuildInputs;
      in
      rec {
        # For `nix build` & `nix run`:
        defaultPackage = packages.bevy_template;
        packages = rec {
          bevy_template = naersk'.buildPackage {
            src = ./.;
            nativeBuildInputs = nativeBuildInputs;
            buildInputs = buildInputs;
            postInstall = ''
              cp -r assets $out/bin/
            '';
            # Disables dynamic linking when building with Nix
            cargoBuildOptions = x: x ++ [ "--no-default-features" ];
          };
        };

        programs.nix-ld = {
          enable = true;
        };
        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = all_deps;
          shellHook = ''
            export WINIT_UNIX_BACKEND=wayland
            export CARGO_MANIFEST_DIR=$(pwd)
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath all_deps}"
          '';
        };
      }
    );
}
