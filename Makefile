all: build run

build:
	@cargo build

run:
	@target/debug/jellysmack-test
