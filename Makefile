compile:
	cd lib && rustc -L. ../src/core/lib.rs
	rustc -Llib src/main.rs -o pong -g

run: compile
	./pong
