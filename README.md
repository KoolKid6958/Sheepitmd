# Sheepit-Manager-CLI

> [!NOTE]
> This project is still actively being developed. Don't expect things to work quite yet.

## What it is
This is a Cli manager for [SheepIt](https://sheepit-renderfarm.com). It is meant to be a version of the Sheepit Manager that I can get working sooner, as I anticipate the other one taking a while.

## Usage
- Clone this repo
- Run `cargo run -r -- -h` to see available options. 

## Goals
- [ ] Auto-restart for the clients if they crash.
- [ ] Multiple clients from one executable.
- [ ] Detect Nvidia driver crashes. The hope would be it could detect the crash, and force restart the machine so it won't hang without any sign of it unless you check the session yourself.

## OS Support
Currently the main focus is for Linux based systems, Windows support is planned, but will come later.