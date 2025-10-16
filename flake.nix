{
  description = "Rust (meta-introspector fork)";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    flake-utils.url = "github:meta-introspector/flake-utils?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, flake-utils } :
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      rec {
        packages.default = pkgs.callPackage ./default.nix { }; # Assuming a default.nix for the actual build
        devShells.default = pkgs.mkShell {
          packages = [
            packages.default
          ];
        };
      });
}