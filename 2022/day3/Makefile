all: run

test:
	@echo No Tests

.PHONY: all test clean

run: build/day3
	@./build/day3

build/day3: main.cpp
	@mkdir -p $(@D)
	g++ -o build/day3 main.cpp

clean:
	rm -rf build/day3

