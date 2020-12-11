#!/bin/env python3
from collections import Counter


def read_input() -> list:
    inp = []
    with open('input', 'r') as f:
        for l in f.readlines():
            inp.append(int(l.replace('\n', '')))
    return inp


def get_diffs(adapters):
    adapters.sort()
    adapters.insert(0, 0)
    adapters.append(adapters[-1] + 3)
    diffs = []
    for a, b in zip(adapters, adapters[1:]):
        diffs.append(b - a)
    return diffs


def solve1():
    adapters = read_input()
    diffs = get_diffs(adapters)
    count = Counter(diffs)
    print(count[1] * count[3])


def solve2():
    adapters = read_input()
    diffs = get_diffs(adapters)
    cons = []
    l = 0
    for d in diffs:
        if d != 1:
            cons.append(l)
            l = 0
        elif d == 1:
            l += 1
    total = 1
    count = Counter(cons)
    for k, v in count.items():
        if k < 2:
            continue
        perms = 0
        if k == 2:
            perms = 2
        if k == 3:
            perms = 4
        if k == 4:
            perms = 7
        total = total * (perms ** v)
    print(total)

if __name__ == "__main__":
    solve1()
    solve2()
