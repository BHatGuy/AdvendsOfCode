DAY = 17

all: run


run: build/day$(DAY)
	@./build/day$(DAY) input.txt

test: build/day$(DAY)
	@./build/day$(DAY) test.txt

benchmark: build/day$(DAY)
	@multitime -q -n 10 ./build/day$(DAY) input.txt

build/day$(DAY): main.cpp
	@mkdir -p $(@D)
	@g++ -o build/day$(DAY) main.cpp

build/day$(DAY)_dbg: main.cpp
	@mkdir -p $(@D)
	g++ -ggdb -O0 -o build/day$(DAY)_dbg main.cpp

debug: build/day$(DAY)_dbg
	cgdb build/day$(DAY)_dbg

clean:
	rm -rf build/

.PHONY: all test clean benchmark debug
