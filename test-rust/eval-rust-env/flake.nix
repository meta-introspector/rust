{
  description = "Evaluates Rust environment paths";

  inputs = {
    nixpkgs.url = "github:meta-introspector/nixpkgs?ref=feature/CRQ-016-nixify";
    rust-overlay.url = "github:meta-introspector/rust-overlay?ref=feature/CRQ-016-nixify";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "aarch64-linux"; # Assuming aarch64-linux for evaluation
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };

      rustToolchain = pkgs.rustChannels.nightly.rust.override {
        targets = [ "aarch64-unknown-linux-gnu" ];
      };

    in
    {
      sccache = pkgs.sccache.outPath;
      curl = pkgs.curl.outPath;
      rustc = "${rustToolchain}/bin/rustc";
      cargo = "${rustToolchain}/bin/cargo";
    };
}
