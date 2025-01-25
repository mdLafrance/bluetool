{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "blueman";
  version = "0.1.0";

  src = pkgs.lib.cleanSource ./.;

  buildInputs = [
      pkgs.dbus
  ];

  nativeBuildInputs = [
    pkgs.pkg-config
  ];

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}

