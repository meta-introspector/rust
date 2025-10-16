{
  description = "Rust (meta-introspector fork)";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        packages.default = pkgs.callPackage ./default.nix { }; # Provides the source
        devShells.default = mkShell {
          name = "rust-src-dev-shell";
          buildInputs = [
            openssl
            glibc.out
            glibc.static
            pkg-config
            python3
            python3Packages.pip
            git
            curl
            cacert
            patchelf
            nix
            # Tools from the example
            eza
            fd
            rust-bin.beta.latest.default # Using rust-bin from rust-overlay
          ];

          # Keep existing environment variables
          RUSTC_ICE = "0";
          LD_LIBRARY_PATH = "${lib.makeLibraryPath [
            stdenv.cc.cc.lib
          ]}";

          # Add shell hooks from the example
          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
        };
      }
    );
}
