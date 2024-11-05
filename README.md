# Spacepix

## Welcome!
Hello and welcome to the Spacepix repository.  Spacepix is educational software that allows the user to explore the NASA API and it's data from a desktop application written in Rust and built on the [egui](https://github.com/emilk/egui) crate.

## Building and Running
Because Spacepix is written in Rust, it should run roughly the same on Linux, Mac, and Windows.

### To Build Spacepix
Run `gh repo clone CodeCanna/spacepix` or `git clone https://github.com/CodeCanna/spacepix.git`

`cd spacepix` to go into the Spacepix directory.
`cargo build` to simply build Spacepix (unoptimized) or `cargo build --release` to run Spacepix with optimizations.  This will output a spacepix executable in `spacepix/output` that you can run.

### To Run Spacepix
If you just want to run Spacepix you can use `cargo run` or `cargo run --release` (optimizations)
