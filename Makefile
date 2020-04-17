.PHONY: all test run doc

all: bin test run  doc

doc:
	RUSTDOCFLAGS="--html-in-header doc/katex.html" cargo doc --no-deps --open

bin: 
	cargo build

test: 
	cargo test

run:
	cargo run