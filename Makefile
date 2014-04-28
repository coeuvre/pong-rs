compile:
	rustc -Llib src/main.rs -o pong -g

run: compile
	./pong
