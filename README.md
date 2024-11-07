# Spacepix

## Welcome!
Hello and welcome to the Spacepix repository.  Spacepix is educational software that allows the user to explore the NASA API and it's data from a desktop application written in Rust and built on the [egui](https://github.com/emilk/egui) crate.

## Building and Running
Because Spacepix is written in Rust, it should run roughly the same on Linux, Mac, and Windows.

### To Build Spacepix
Run `gh repo clone CodeCanna/spacepix` or `git clone https://github.com/CodeCanna/spacepix.git`

Spacepix uses `Make` and `cargo` to build and run:
* `make all` Builds optimized for both Linux and Windows x64 targets (MAC to come) uses `--release`
* `make linux` Builds for both Linux and Windows optmized uses `--release`
* `make linuxdev` Builds for Linux x64 unoptmized
* `make windows` Builds for Windows x64 optimized uses `--release`
* `make windowsdev` Builds for Windows x64 unoptimized


### To Run Spacepix
If you just want to run Spacepix you can use `cargo run` or `cargo run --release` (optimizations)
