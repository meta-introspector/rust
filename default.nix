{
  pkgs ? import <nixpkgs> {}
}:

let
  x = pkgs.callPackage (./src/tools/nix-dev-shell/x) {};
in

pkgs.stdenv.mkDerivation {
  pname = "rust";
  version = "solana-tools-v1.51"; # Placeholder version

  src = ./.; # Source is the current directory

  nativeBuildInputs = with pkgs; [
    binutils cmake ninja pkg-config python3 git curl cacert patchelf nix
  ];

  preConfigure = ''
    mkdir -p $TMPDIR/bin
    ln -s ${pkgs.python3}/bin/python3 $TMPDIR/bin/python
    export PATH=$TMPDIR/bin:$PATH
  '';

  buildInputs = with pkgs; [
    openssl glibc.out glibc.static x # Include x here
  ];

  buildPhase = ''
    ./x.py build --stage 1 --target ${pkgs.stdenv.hostPlatform.config},sbf-solana-solana,sbpf-solana-solana,sbpfv1-solana-solana,sbpfv2-solana-solana,sbpfv3-solana-solana,sbpfv4-solana-solana
  '';

  installPhase = ''
    # This will be complex, need to analyze the output of x.py build
    # For now, let's just copy the entire build directory
    mkdir -p $out/
    cp -r build/ $out/
  '';
}