build:
	cargo build --release

clean:
	rm -rf ./target

run *args:
	cargo run -- {{ args }}

arch:
	cargo zigbuild --target aarch64-apple-darwin --release
	cargo zigbuild --target x86_64-pc-windows-gnu --release
	cargo zigbuild --target x86_64-unknown-freebsd --release
	cargo zigbuild --target x86_64-unknown-linux-gnu --release