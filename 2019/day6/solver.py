#!/bin/env python3

def read_input() -> [int]:
    inp = []
    with open("input", "r") as f:
        for line in f.readlines():
            pair = tuple(line.replace('\n', '').split(')'))
            inp.append(pair)
    return inp


def build_dict(omap):
    d = dict()
    for p, c in omap:
        if p not in d:
           d[p] = {c}
        else:
            d[p] |= {c}
    return d

def count(d, x):
    c = 0
    for 


def solve1():
    omap = read_input()
    d = build_dict(omap)
    res = count(d, 'COM')


def solve2():
    pass


if __name__ == "__main__":
    solve1()
    solve2()
