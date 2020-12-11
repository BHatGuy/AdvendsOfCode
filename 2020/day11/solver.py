#!/bin/env python3
from copy import deepcopy


def read_input(file='input') -> list:
    inp = []
    with open(file, 'r') as f:
        for l in f.readlines():

            inp.append(list(l.replace('\n', '')))
    return inp


def count_neighbours(x, y, area):
    count = 0
    for dx in (-1, 0, 1):
        for dy in (-1, 0, 1):
            if dx == dy == 0:
                continue
            if not 0 <= x + dx < len(area):
                continue
            if not 0 <= y + dy < len(area[x]):
                continue
            if area[x + dx][y + dy] == '#':
                count += 1
    return count


def count_far_neighbours(x, y, area):
    count = 0
    for dx in (-1, 0, 1):
        for dy in (-1, 0, 1):
            if dx == dy == 0:
                continue
            i = 1
            while True:
                if not 0 <= x + i * dx < len(area):
                    break
                if not 0 <= y + i * dy < len(area[x]):
                    break
                if area[x + i * dx][y + i * dy] == 'L':
                    break
                if area[x + i * dx][y + i * dy] == '#':
                    count += 1
                    break
                i += 1
    return count


def print_area(area):
    for row in area:
        print("".join(row))
    print()


def solve1():
    area = read_input()
    changes = True
    while changes:
        changes = False
        newarea = deepcopy(area)
        for x in range(len(area)):
            for y in range(len(area[x])):
                seat = area[x][y]
                if seat == '.':
                    continue
                if seat == 'L' and count_neighbours(x, y, area) == 0:
                    newarea[x][y] = '#'
                    changes = True
                if seat == '#' and count_neighbours(x, y, area) >= 4:
                    newarea[x][y] = 'L'
                    changes = True
        area = newarea
    count = 0
    for row in area:
        for seat in row:
            if seat == '#':
                count += 1
    return count


def solve2():
    area = read_input()
    changes = True
    while changes:
        changes = False
        newarea = deepcopy(area)
        for x in range(len(area)):
            for y in range(len(area[x])):
                seat = area[x][y]
                if seat == '.':
                    continue
                if seat == 'L' and count_far_neighbours(x, y, area) == 0:
                    newarea[x][y] = '#'
                    changes = True
                if seat == '#' and count_far_neighbours(x, y, area) >= 5:
                    newarea[x][y] = 'L'
                    changes = True
        area = newarea
    count = 0
    for row in area:
        for seat in row:
            if seat == '#':
                count += 1
    return count


if __name__ == "__main__":
    print(solve1())
    print(solve2())
