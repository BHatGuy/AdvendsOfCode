import itertools
from itertools import product


def get_input(name="input.txt"):
    lines = []
    with open(name, "r") as f:
        lines = f.readlines()
    res = []
    scanner = set()
    for l in lines:
        l = l.strip()
        if len(l) == 0:
            res.append(scanner)
            scanner = set()
            continue
        if "--- scanner " in l:
            continue
        scanner.add(tuple(int(x) for x in l.split(",")))
    res.append(scanner)
    return res


def facing(scanner, n):
    shifted = set()
    for l in scanner:
        shifted.add(tuple(itertools.islice(itertools.cycle(l), n, n + len(l))))
    return shifted


def sub(a, b):
    return tuple(map(lambda a: a[0] - a[1], zip(a, b)))


def count_overlap(s1, s2):
    return len(s1.intersection(s2))

def add_single(p, d):
    return tuple(map(lambda a: a[0] + a[1], zip(p, d)))

def add(scanner, d):
    added = set()
    for s in scanner:
        added.add(add_single(s, d))
    return added


def rotate(scanner, d):
    rotated = set()
    for s in scanner:
        rotated.add(tuple(map(lambda a: a[0] * a[1], zip(s, d))))
    return rotated


def rotations():
    vecs = {(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1),
            (0, 0, -1)}

    for a in vecs:
        for b in vecs.difference({a}):
            for c in vecs.difference({a, b}):
                m = [a, b, c]
                if det(m) == 1: # idk why
                    yield m

def mult(m, p):
    res = []
    for row in m:
        acc = 0
        for a, b in zip(row, p):
            acc += a * b
        res.append(acc)
    return tuple(res)

def rotate(r, scanner):
    rotated = set()
    for s in scanner:
        rotated.add(mult(r, s))
    return rotated

def det(m):
    return m[0][0] * m[1][1] * m[2][2] + m[0][1] * m[1][2] * m[2][0] + m[0][
        2] * m[1][0] * m[2][1] - m[0][2] * m[1][1] * m[2][0] - m[0][1] * m[1][
            0] * m[2][2] - m[0][0] * m[1][2] * m[2][1]




def solve(inp):
    scanners = []
    acc = inp.pop()
    while len(inp) > 0:
        scanner = inp.pop()
        for r in rotations():
            rotated = rotate(r, scanner)
            for a, b in product(acc, rotated):
                d = sub(a, b)
                shifted = add(rotated, d)
                if count_overlap(acc, shifted) >= 12:
                    scanners.append(add_single((0, 0, 0), d))
                    acc |= shifted
                    print("remaining:", len(inp))
                    break
            else:
                continue
            break
        else:
            inp.insert(0, scanner)
    print(len(acc))
    m = max(sum(map(abs, sub(a, b))) for a,b in product(scanners, scanners))
    print(m)

def solve2(numbers):
    pass


solve(get_input())
