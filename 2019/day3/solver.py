#!/bin/env python3


# parsing like this is probably unsave as hell!
def read_input():
    cable1 = []
    cable2 = []
    with open('input', 'r') as f:
        c1, c2, _ = f.read().split('\n')
        cable1 = list(map(lambda x: (x[0], int(x[1:])), c1.split(',')))
        cable2 = list(map(lambda x: (x[0], int(x[1:])), c2.split(',')))

    return cable1, cable2


def do_steps(c1, c2):
    s1 = set()
    s2 = set()

    x = 0
    y = 0
    for d, l in c1:
        dx = 0
        dy = 0
        if d == 'U':
            dy = 1
        elif d == 'D':
            dy = -1
        elif d == 'R':
            dx = 1
        elif d == 'L':
            dx = -1
        for _ in range(l):
            x += dx
            y += dy
            s1.add((x, y))

    x = 0
    y = 0
    for d, l in c2:
        dx = 0
        dy = 0
        if d == 'U':
            dy = 1
        elif d == 'D':
            dy = -1
        elif d == 'R':
            dx = 1
        elif d == 'L':
            dx = -1
        for _ in range(l):
            x += dx
            y += dy
            s2.add((x, y))

    return s1, s2


def get_distance(c1, c2, goal):
    distance = 0
    ix, iy = goal
    x = 0
    y = 0
    for d, l in c1:
        dx = 0
        dy = 0
        if d == 'U':
            dy = 1
        elif d == 'D':
            dy = -1
        elif d == 'R':
            dx = 1
        elif d == 'L':
            dx = -1
        for _ in range(l):
            if x == ix and y == iy:
                break
            x += dx
            y += dy
            distance += 1
            

    x = 0
    y = 0
    for d, l in c2:
        dx = 0
        dy = 0
        if d == 'U':
            dy = 1
        elif d == 'D':
            dy = -1
        elif d == 'R':
            dx = 1
        elif d == 'L':
            dx = -1
        for _ in range(l):
            if x == ix and y == iy:
                break
            x += dx
            y += dy
            distance += 1
            

    return distance


def solve1():
    steps = read_input()
    s1, s2 = do_steps(*steps)
    return min(map(lambda x: abs(x[0]) + abs(x[1]), s1.intersection(s2)))


def solve2():
   c1, c2 = read_input()
   s1, s2 = do_steps(c1, c2)
   return min(map(lambda x: get_distance(c1, c2, x), s1.intersection(s2)))


if __name__ == "__main__":
    print(solve1())
    print(solve2())
