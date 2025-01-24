{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    dbus
    pkg-config
  ];

  shellHook = ''
    export RUST_BACKTRACE=1
  '';
}

