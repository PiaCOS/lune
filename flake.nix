{
  description = "Rust Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };
      rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
        extensions = [ "rust-src" "rust-analyzer" ];
      };
    in {
      # Lune
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "lune";
        version = "0.1.0";
        src = ./.;

        nativeBuildInputs = [ rustToolchain pkgs.pkg-config ];
        buildInputs = [ pkgs.openssl ];

        cargoLock = {
          lockFile = ./Cargo.lock;
        };
      };

      # DEV SHELL
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [
          rustToolchain
          pkgs.openssl
        ];
      };
    };
}

