all: run

test:
	@echo No Tests

.PHONY: all test clean

run: build/day1
	@./build/day1

build/day1: main.cpp
	@mkdir -p $(@D)
	g++ -o build/day1 main.cpp

clean:
	rm -rf build


