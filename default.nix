{ pkgs ? import <nixpkgs> { } }:

let
toml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
version = toml.package.version;
pname = toml.package.name;
in
pkgs.rustPlatform.buildRustPackage rec {
  inherit pname;
  inherit version;

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

