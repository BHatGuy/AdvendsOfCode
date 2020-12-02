from operator import mul, add
from itertools import combinations, permutations


def read_input():
    inp = []
    with open("input", "r") as f:
        s = f.read().strip('\n')
        inp = s.split(',')
    inp = list(map(int, inp))
    return inp


def run(mem, in1, in2):
    mem[1] = in1
    mem[2] = in2

    pc = 0
    while True:
        opcode = mem[pc]
        a = mem[pc + 1]
        b = mem[pc + 2]
        c = mem[pc + 3]

        op = None
        if opcode == 99:
            break
        elif opcode == 1:
            op = add
        elif opcode == 2:
            op = mul
        else:
            raise Exception("Epic Exception!")

        mem[c] = op(mem[a], mem[b])

        pc += 4

    return mem[0]


def solve1():
    prog = read_input()
    return run(prog, 12, 2)



def solve2():
    initial_mem = read_input()
    nums = list(range(100)) * 2
    for n, v in combinations(nums, 2):
        mem = initial_mem.copy()
        res = run(mem, n, v)
        if res == 19690720:
            return n * 100 + v


if __name__ == "__main__":
    print(solve1())
    print(solve2())
