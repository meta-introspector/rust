{ pkgs ? import <nixpkgs> { }
}:

pkgs.stdenv.mkDerivation {
  pname = "rust";
  version = "solana-tools-v1.51"; # Placeholder version

  src = ./.; # Source is the current directory

  nativeBuildInputs = [
    pkgs.python3 # Add python3 to the build environment
  ];

  buildPhase = ''
    echo "Starting Rust build using x.py..."
    rm -f config.toml # Remove existing config.toml
    python x.py build
  '';

  installPhase = ''
    mkdir -p $out/share/rust
    cp -r . $out/share/rust/
  '';
}
