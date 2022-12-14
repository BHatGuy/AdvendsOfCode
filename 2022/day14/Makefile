DAY = 14

all: run


run:
	@python solve.py input.txt

test:
	@python solve.py test.txt

benchmark:
	@multitime -q -n 10 python solve.py input.txt


.PHONY: all test clean benchmark
