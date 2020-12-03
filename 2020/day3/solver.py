#!/usr/bin/env python3
from functools import reduce
from operator import mul
def read_input():
    inp = []
    with open('input', 'r') as f:
        for line in f.readlines():
            line = line.strip('\n')
            row = []
            for c in line:
                row.append(c)
            inp.append(row)
    return inp


def count_trees(stepx, stepy):
    treemap = read_input()
    x = 0
    y = 0
    count = 0
    while y < len(treemap):
        if x >= len(treemap[y]):
            x -= len(treemap[y])
        if treemap[y][x] == '#':
            count += 1
        x += stepx
        y += stepy

    return count


def solve1():
    return count_trees(3, 1)


def solve2():
    slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    counts = [count_trees(*x) for x in slopes]
    return reduce(mul, counts, 1)


if __name__ == "__main__":
    print(solve1())
    print(solve2())
