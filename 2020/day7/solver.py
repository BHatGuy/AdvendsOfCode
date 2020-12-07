#!/bin/env python3
from functools import reduce

SG = "shiny gold"

def read_input() -> dict:
    rules = dict()
    with open('input', 'r') as f:
        for rule in f.readlines():
            outer, inners = rule.replace('\n', '').split(' contain ')
            outer = outer.replace(' bags', '')
            inn = dict()
            for inner in inners.split(', '):
                if inner[0] == 'n':
                    break
                amount = int(inner[0])
                color = inner[2:].replace('.', '').replace(' bags', '').replace(' bag', '')
                inn[color] = amount
            rules[outer] = inn

    return rules

def can_contain(bag, rules):
    if SG in rules[bag].keys():
        return True
    else:
        for b in rules[bag].keys():
            if can_contain(b, rules):
                return True
    return False

def count_rec(bag, rules):
    count = 0
    for b in rules[bag].keys():
        count += rules[bag][b] * (count_rec(b, rules) + 1)
    return count

def solve1():
    rules = read_input()
    count = len(list(filter(lambda x: can_contain(x, rules), rules.keys())))
    print(count)


def solve2():
    rules = read_input()
    print(count_rec(SG, rules))


if __name__ == "__main__":
    solve1()
    solve2()
