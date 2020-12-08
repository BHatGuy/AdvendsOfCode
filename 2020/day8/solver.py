#!/bin/env python3
from functools import reduce

SG = "shiny gold"


def read_input() -> dict:
    inp = []
    with open('input', 'r') as f:
        for l in f.readlines():
            op, arg = l.replace('\n', '').split(' ')
            inp.append([op, int(arg)])
    return inp


def run(prog):
    count = set()
    pc = 0
    acc = 0

    while pc < len(prog):
        if pc in count:
            return (False, acc)
        count.add(pc)
        op, arg = prog[pc]
        if op == 'acc':
            acc += arg
            pc += 1
        elif op == 'jmp':
            pc += arg
        elif op == 'nop':
            pc += 1
        else:
            raise Exception("Unknown operation")
    return (True, acc)


def solve1():
    prog = read_input()
    print(run(prog))


def solve2():
    prog = read_input()
    for inst in prog:

        if inst[0] == 'nop':
            inst[0] = 'jmp'
        elif inst[0] == 'jmp':
            inst[0] = 'nop'

        res = run(prog)
        if res[0]:
            print(res)
            return

        if inst[0] == 'nop':
            inst[0] = 'jmp'
        elif inst[0] == 'jmp':
            inst[0] = 'nop'


if __name__ == "__main__":
    solve1()
    solve2()
