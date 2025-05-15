{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "zenos-dev";

  buildInputs = with pkgs; [
    rustup
    lld                     
    qemu                   
    nasm                    
  ];

  shellHook = ''
    export RUST_BACKTRACE=1
    rustup default nightly
    rustup target add x86_64-unknown-none
    rustup component add rust-src --toolchain nightly
    echo "✨ Entered no_std shell with nightly Rust and x86_64-unknown-none target."
  '';
}

