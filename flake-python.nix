{
  description = "Python development environment extending rust-src";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    rustSrcFlake.url = "github:meta-introspector/rust?ref=4584acb6a6f"; # Reference the committed rust-src flake
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, rustSrcFlake }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
        rustToolchain = pkgs.rustChannels.nightly.rust.override { targets = [ "aarch64-unknown-linux-gnu" ]; };
      in
      rec {
        devShells.default = pkgs.mkShell {
          name = "python-rust-dev-shell";

          # Inherit packages and environment from the rust-src devShell
          # We need to explicitly list them or find a way to merge the devShells
          # For now, let's explicitly list the core components and add Python
          packages = [
            rustToolchain
            pkgs.python3
            pkgs.python3Packages.pip
            pkgs.python3Packages.venv
          ];

          nativeBuildInputs = [
            pkgs.binutils
            pkgs.cmake
            pkgs.ninja
            pkgs.pkg-config
            pkgs.git
            pkgs.curl
            pkgs.cacert
            pkgs.patchelf
            pkgs.nix
          ];

          buildInputs = [
            pkgs.openssl
            pkgs.glibc.out
            pkgs.glibc.static
          ];

          RUSTC_ICE = "0";
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [
            pkgs.stdenv.cc.cc.lib
          ]}";
        };
      });
}
