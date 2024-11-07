all:
	cargo build --release && cargo build --target x86_64-pc-windows-gnu --release

run:
	cargo run

rrun:
	cargo run --release

linux:
	cargo build --release

linuxdev:
	cargo build

windows:
	cargo build --target x86_64-pc-windows-gnu --release

windowsdev:
	cargo build --target x86_64-pc-windows-gnu

