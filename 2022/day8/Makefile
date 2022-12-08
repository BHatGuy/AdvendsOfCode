DAY = 8

all: run


run: build/day$(DAY)
	@./build/day$(DAY) input.txt

test: build/day$(DAY)
	@./build/day$(DAY) test.txt

benchmark: build/day$(DAY)
	@multitime -q -n 10 ./build/day$(DAY) input.txt

build/day$(DAY): main.cpp
	@mkdir -p $(@D)
	g++ -std=c++20 -Wall -o build/day$(DAY) main.cpp

clean:
	rm -rf build/

.PHONY: all test clean benchmark
