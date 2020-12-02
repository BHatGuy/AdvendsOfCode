#!/usr/bin/env python3
import itertools
from operator import mul
from functools import reduce


def read_input():
    res = []
    with open("day1.inp", "r") as f:
        for line in f.readlines():
            res.append(int(line))
    return res


def special_sum(x):
    return (sum(x), x)


numbers = read_input()

# Change length for the respective task
combis = itertools.combinations(numbers, 3)

sums = map(special_sum, combis)

res = filter((lambda a: a[0] == 2020), sums)

triple = list(res)[0][1]
prod = reduce(mul, triple)
print(prod)
