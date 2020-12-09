#!/bin/env python3
from itertools import combinations

SG = "shiny gold"


def read_input() -> dict:
    inp = []
    with open('input', 'r') as f:
        for l in f.readlines():
            inp.append(int(l.replace('\n', '')))
    return inp

def find_invalid(numbers):
    numbers = read_input()
    
    nosum = False
    while not nosum:
        window = numbers[0:25]
        goal = numbers[25]
        for combi in combinations(window, 2):
            if sum(combi) == goal:
                break
        else:
            return goal
        numbers = numbers[1:]

def solve1():
    print(find_invalid(read_input()))



def solve2():
    numbers = read_input()
    inval = find_invalid(numbers)

    for starti in range(len(numbers) - 1):
        for length in range(2, len(numbers) - starti):
            window = numbers[starti:starti+length]
            if sum(window) == inval:
                print(min(window) + max(window))
                return


if __name__ == "__main__":
    solve1()
    solve2()
