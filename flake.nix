{
  description = "Factorio utilities — recipe calculator, blueprint tooling, factory planner";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
            "rustfmt"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
            pkgs.pkg-config
            pkgs.cargo-nextest
            pkgs.z3
            pkgs.llvmPackages.libclang
          ];
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          BINDGEN_EXTRA_CLANG_ARGS = "-I${pkgs.z3.dev}/include";

          shellHook = ''
            zsh
          '';
        };

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
