all: run


run: build/day4
	@./build/day4 input.txt

test: build/day4
	@./build/day4 test.txt

benchmark: build/day4
	@multitime -q -n 10 ./build/day4 input.txt

build/day4: main.cpp
	@mkdir -p $(@D)
	g++ -o build/day4 main.cpp

clean:
	rm -rf build/day4

.PHONY: all test clean benchmark
