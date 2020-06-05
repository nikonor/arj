build:
	cargo build

help: build
	./target/debug/arj --help

init: build
	./target/debug/arj --init .
