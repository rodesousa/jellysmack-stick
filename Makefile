all: build run

build:
	@cargo build

run:
	@target/debug/jellysmack-test

easy:
	@cargo run < dataset/easy1.txt
	@cargo run < dataset/easy2.txt
	@cargo run < dataset/easy3.txt
	@cargo run < dataset/easy4.txt

special:
	@cargo run < dataset/wolo.txt
	@cargo run < dataset/lol.txt
