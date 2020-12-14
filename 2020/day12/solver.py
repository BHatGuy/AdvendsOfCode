#!/bin/env python3
from math import sin, cos, radians


def read_input(file='input'):
    inp = []
    with open(file, 'r') as f:
        for l in f.readlines():
            l = l.replace('\n', '')

            inp.append((l[0], int(l[1:])))
    return inp


def solve1():
    data = read_input()
    x = 0
    y = 0
    d = 0
    for inst, val in data:
        if inst == 'R':
            d -= val
        elif inst == 'L':
            d += val
        elif inst == 'N':
            y += val
        elif inst == 'E':
            x += val
        elif inst == 'S':
            y -= val
        elif inst == 'W':
            x -= val
        elif inst == 'F':
            x += int(cos(radians(d))) * val
            y += int(sin(radians(d))) * val
        else:
            raise Exception("WTF")
    return abs(x) + abs(y)



def solve2():
    data = read_input(file='test_input')
    x = 0
    y = 0
    d = 0
    for inst, val in data:
        if inst == 'R':
            d -= val
        elif inst == 'L':
            d += val
        elif inst == 'N':
            y += val
        elif inst == 'E':
            x += val
        elif inst == 'S':
            y -= val
        elif inst == 'W':
            x -= val
        elif inst == 'F':
            x += int(cos(radians(d))) * val
            y += int(sin(radians(d))) * val
        else:
            raise Exception("WTF")
    return abs(x) + abs(y)


if __name__ == "__main__":
    print(solve1())
    print(solve2())
