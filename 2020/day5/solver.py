#!/bin/env python3
def read_input():
    inp = []
    with open('input', 'r') as f:
        for line in f.readlines():
            inp.append(line)
    return inp

def convert(word):
    word = word.strip('\n')
    word = word.replace('B', '1').replace('R', '1')
    word = word.replace('F', '0').replace('L', '0')
    return int(word, 2)

def solve1():
    seats = read_input()
    return max(map(convert, seats))


def solve2():
    ids = list(map(convert, read_input()))
    ids.sort()
    last = ids[0] - 1
    for i in ids:
        if i == last + 2:
            return last + 1
        last = i

if __name__ == "__main__":
    print(solve1())
    print(solve2())
