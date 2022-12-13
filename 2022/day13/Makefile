DAY = 13

all: run


run:
	python solve.py input.txt

test:
	python solve.py test.txt

benchmark: build/day$(DAY)
	@multitime -q -n 10 python solve.py input.txt

clean:
	rm -rf build/

.PHONY: all test clean benchmark
