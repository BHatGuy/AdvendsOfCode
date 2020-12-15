#!/bin/env python3
from functools import reduce
import sys


def read_input(file='input'):
    with open(file, 'r') as f:
        a = int(f.readline())
        ids = []
        for i in f.readline().split(','):
            if i != 'x':
                ids.append(int(i))
            else:
                ids.append('x')

    return a, ids


def solve1():
    arr, ids = read_input()
    waitlist = []
    for t in ids:
        if type(t) is not int:
            continue
        rest = arr % t
        wait = t - rest
        waitlist.append((wait, t))
    res = reduce(lambda x, y: x
                 if x[0] < y[0] else y, waitlist, (sys.maxsize, 0))
    return res[0] * res[1]


def solve2():
    _, ids = read_input()
    ts = ids[0] - (100000000000000 % ids[0]) + 100000000000000
    while True:
        print(ts)
        for i, ID in enumerate(ids):
            if type(ID) is not int:
                continue
                
            if (ts + i) % ID != 0:
                break
        else:
            return ts
        ts += ids[0]


if __name__ == "__main__":
    print(solve1())
    print(solve2())
