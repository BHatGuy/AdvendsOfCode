from pprint import pprint
from tabulate import tabulate
import functools
import operator


def get_input(name="input.txt"):
    lines = []
    with open(name, "r") as f:
        lines = f.readlines()

    res = []
    for l in lines:
        start, end = l.strip().split("->")
        startx, starty = start.split(",")
        endx, endy = end.split(",")
        res.append(((int(startx), int(starty)), (int(endx), int(endy))))

    return res


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1])


def add(a, b):
    return (a[0] + b[0], a[1] + b[1])


def norm_max(s):
    m = max(abs(s[0]), abs(s[1]))
    return (s[0] // m, s[1] // m)


def solve1(inp):
    maxx = 0
    maxy = 0
    for l in inp:
        start, end = l
        if start[0] > maxx:
            maxx = start[0]
        if end[0] > maxx:
            maxx = end[0]
        if start[1] > maxy:
            maxy = start[1]
        if end[1] > maxy:
            maxy = end[1]
    matrix = [[0 for _ in range(maxx + 1)] for _ in range(maxy + 1)]
    for l in inp:
        start, end = l
        slope = sub(end, start)
        if not (slope[0] == 0 or slope[1] == 0):
            continue
        norm_slope = norm_max(slope)
        p = start
        matrix[p[1]][p[0]] += 1
        while p != end:
            p = add(p, norm_slope)
            matrix[p[1]][p[0]] += 1
    flat = functools.reduce(operator.iconcat, matrix, [])

    return sum(map(lambda x: 1 if x > 1 else 0, flat))


def solve2(inp):
    maxx = 0
    maxy = 0
    for l in inp:
        start, end = l
        if start[0] > maxx:
            maxx = start[0]
        if end[0] > maxx:
            maxx = end[0]
        if start[1] > maxy:
            maxy = start[1]
        if end[1] > maxy:
            maxy = end[1]
    matrix = [[0 for _ in range(maxx + 1)] for _ in range(maxy + 1)]
    for l in inp:
        start, end = l
        slope = sub(end, start)
        norm_slope = norm_max(slope)
        p = start
        matrix[p[1]][p[0]] += 1
        while p != end:
            p = add(p, norm_slope)
            matrix[p[1]][p[0]] += 1
    flat = functools.reduce(operator.iconcat, matrix, [])

    return sum(map(lambda x: 1 if x > 1 else 0, flat))


if __name__ == "__main__":
    print(solve1(get_input()))
    print(solve2(get_input()))