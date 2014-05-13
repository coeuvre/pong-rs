compile:
	rustc --out-dir=lib -Llib src/game/lib.rs
	rustc -Llib src/main.rs -o pong -g

run: compile
	./pong
