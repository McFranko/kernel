

run:
	qemu-system-x86_64 target/target/debug/bootimage-kernel.bin

build:
	cargo +nightly bootimage --target target.json

buildrun:
	make build
	make run
