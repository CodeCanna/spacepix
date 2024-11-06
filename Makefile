all:
	cargo build && cargo build --target x86_64-pc-windows-gnu

linux:
	cargo build --release

linuxdev:
	cargo build

windows:
	cargo build --target x86_64-pc-windows-gnu

windowsdev:
	cargo build --target x86_64-pc-windows-gnu --release

