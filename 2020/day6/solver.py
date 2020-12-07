#!/bin/env python3
from functools import reduce


def read_input() -> [[str]]:
    groups = []
    with open('input', 'r') as f:
        for group in f.read().split('\n\n'):
            g = []
            for pers in group.splitlines():
                g.append(list(pers))
            groups.append(g)

    return groups


def solve1():
    counts = [
        len(reduce(lambda x, y: x | y, map(set, g), set()))
        for g in read_input()
    ]
    print(sum(counts))
    


def solve2():
    counts = [
        len(set.intersection(*map(set, g)))
        for g in read_input()
    ]
    print(sum(counts))


if __name__ == "__main__":
    solve1()
    solve2()
