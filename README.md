<h1 align=center>
  BlueTool
</h1>
<h3 align=center>
  A TUI Bluetooth device manager
</h3>
<div align=center>

  [![Pipeline](https://github.com/mdLafrance/bluetool/actions/workflows/pipeline.yaml/badge.svg)](https://github.com/mdLafrance/bluetool/actions/workflows/pipeline.yaml)
  [![crates.io](https://img.shields.io/crates/v/bluetool)](https://crates.io/crates/bluetool)

</div>
<br />

A terminal gui app for linux Bluez systems, meant to be easier to use than `bluetoothctl`, and quicker than a traditional GUI.

## Usage
Run `bluetool` from the terminal to drop into the gui. Press **q** at any time to quit.

Commands keys are displayed along the bottom of the window (see screenshot below). `bluetool` will continually listen for new devices and device updates.

## Installation
### Cargo
Install with `cargo`:
```bash
cargo install bluetool
bluetool --help
```
This builds and installs the `bluetool` binary.

### Nix
If you're on nix (like me), I've included a [nix flake](./default.nix) in the package you can use to run `bluetool` or install it.
> I'm currently working on adding this to nixpkgs

### Arch
> I'm working on adding this to the AUR, since once of my machines is on arch. 
