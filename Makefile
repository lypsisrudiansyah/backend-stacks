run: build
	./target/release/weatherApi 38 -77

build:
	cargo build --release

PHONY: run build