#!/bin/env python3


def read_input() -> [int]:
    inp = []
    with open("input", "r") as f:
        s = f.read().strip('\n')
        inp = s.split(',')
    inp = list(map(int, inp))
    return inp


def run(mem: [int], inp: [int]) -> [int]:
    out = []

    pc = 0
    while True:
        op = str(mem[pc]).zfill(5)
        opcode = op[3:]
        mode_c = op[0]
        mode_b = op[1]
        mode_a = op[2]

        if opcode == '99':
            break
        elif opcode == '01':
            a = mem[pc + 1]
            b = mem[pc + 2]
            c = mem[pc + 3]

            if mode_a == '0':
                a = mem[a]
            if mode_b == '0':
                b = mem[b]
            mem[c] = a + b
            pc += 4
        elif opcode == '02':
            a = mem[pc + 1]
            b = mem[pc + 2]
            c = mem[pc + 3]
            if mode_a == '0':
                a = mem[a]
            if mode_b == '0':
                b = mem[b]
            mem[c] = a * b
            pc += 4
        elif opcode == '03':
            a = mem[pc + 1]
            mem[a] = inp[0]
            inp = inp[1:]
            pc += 2
        elif opcode == '04':
            a = mem[pc + 1]
            if mode_a == '0':
                a = mem[a]
            out.append(a)
            pc += 2
        elif opcode == '05':
            a = mem[pc + 1]
            b = mem[pc + 2]
            if mode_a == '0':
                a = mem[a]
            if mode_b == '0':
                b = mem[b]
            if a != 0:
                pc = b
            else: 
                pc += 3
        elif opcode == '06':
            a = mem[pc + 1]
            b = mem[pc + 2]
            if mode_a == '0':
                a = mem[a]
            if mode_b == '0':
                b = mem[b]
            if a == 0:
                pc = b
            else: 
                pc += 3
        elif opcode == '07':
            a = mem[pc + 1]
            b = mem[pc + 2]
            c = mem[pc + 3]
            if mode_a == '0':
                a = mem[a]
            if mode_b == '0':
                b = mem[b]
            if a < b:
                mem[c] = 1
            else:
                mem[c] = 0
            pc += 4
        elif opcode == '08':
            a = mem[pc + 1]
            b = mem[pc + 2]
            c = mem[pc + 3]
            if mode_a == '0':
                a = mem[a]
            if mode_b == '0':
                b = mem[b]
            if a == b:
                mem[c] = 1
            else:
                mem[c] = 0
            pc += 4
        else:
            raise Exception("Unknown Opcode!")

    return out


def solve1():
    mem = read_input()
    res = run(mem, [1])
    print(res)


def solve2():
    mem = read_input()
    res = run(mem, [5])
    print(res)


if __name__ == "__main__":
    solve1()
    solve2()
