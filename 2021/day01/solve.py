
from functools import reduce

def get_input():
    lines = []
    with open("inp.txt", "r") as f:
        lines = f.readlines()

    return [int(x) for x in lines]

def solve1(inp):

    pairs = [inp[i:i+2] for i in range(len(inp)-1)]
    c = 0
    for p in pairs:
        if p[0] < p[1]:
            c += 1
    return c

def solve2(inp):
    sums = [sum(inp[i:i+3]) for i in range(len(inp)-2)]
    
    pairs = [sums[i:i+2] for i in range(len(sums)-1)]
    c = 0
    for p in pairs:
        if p[0] < p[1]:
            c += 1
    return c

if __name__ == "__main__":
    print(solve1(get_input()))
    print(solve2(get_input()))