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
            "rust-std"
          ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # Librerías nativas mínimas
        nativeBuildInputs = with pkgs; [
          pkg-config
          openssl
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.dioxus-cli
          ]
          ++ nativeBuildInputs;
        };
      }
    );
}
