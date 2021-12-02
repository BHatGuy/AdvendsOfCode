def get_input():
    lines = []
    with open("input.txt", "r") as f:
        lines = f.readlines()
    splits = [l.split(" ") for l in lines]
    res = [(t[0], int(t[1])) for t in splits]
    return res

def solve1():
    x = 0
    depth = 0
    inp = get_input()
    for d in inp:
        if d[0] == "up":
            depth -= d[1]
        if d[0] == "down":
            depth += d[1]
        if d[0] == "forward":
            x += d[1]
    return x * depth

def solve2():
    x = 0
    aim = 0
    depth = 0
    inp = get_input()
    for d in inp:
        if d[0] == "up":
            aim -= d[1]
        if d[0] == "down":
            aim += d[1]
        if d[0] == "forward":
            x += d[1]
            depth += aim*d[1]
    return x * depth

print(solve1())       
print(solve2())       
    