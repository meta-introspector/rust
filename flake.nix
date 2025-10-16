{
  description = "Rust (meta-introspector fork)";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
        rustToolchain = pkgs.rustChannels.nightly.rust.override { targets = [ "aarch64-unknown-linux-gnu" ]; };
      in
      rec {
        packages.default = pkgs.callPackage ./default.nix { }; # Provides the source
        devShells.default = with pkgs; mkShell {
          name = "rust-src-dev-shell";
          packages = [
            rustToolchain # This should include rustc, cargo, and rust-std for the target
            pkgs.python3
          ];
          # Add other nativeBuildInputs and buildInputs if needed for development
          nativeBuildInputs = [
            binutils
            cmake
            ninja
            pkg-config
            pkgs.python3
            pkgs.python3Packages.pip
            pkgs.python3Packages.venv
            git
            curl
            cacert
            patchelf
            nix
          ];
          buildInputs = [
            openssl
            glibc.out
            glibc.static
          ];
          RUSTC_ICE = "0";
          LD_LIBRARY_PATH = "${lib.makeLibraryPath [
            stdenv.cc.cc.lib
          ]}";
        };
      });
}
