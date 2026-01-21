{
  description = "The vocaloid app to empower communities";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Definimos tu toolchain de Rust específico
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
          ];
          targets = [ "wasm32-unknown-unknown" ]; # Importante para Dioxus Web
        };

        # Librerías nativas necesarias para Linux (GTK, OpenSSL, etc.)
        nativeBuildInputs = with pkgs; [
          pkg-config
          openssl
          glib
          gtk3
          libsoup_3
          webkitgtk_4_1
          xdotool
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.dioxus-cli
            pkgs.nil
            pkgs.nixd
            pkgs.nixpkgs-fmt
          ]
          ++ nativeBuildInputs;

          # Configuración para que el linker encuentre las librerías
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeBuildInputs;
        };
      }
    );
}
