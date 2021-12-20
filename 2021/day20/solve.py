from itertools import count
from pprint import pprint
from tabulate import tabulate

def get_input(name="input.txt"):
    lines = []
    with open(name, "r") as f:
        lines = f.readlines()
    algorithm = list(lines[0].strip())
    res = []
    for l in lines[2:]:
        
        res.append(list(l.strip()))

    return algorithm, res


def pad(img, c):
    img.insert(0, [c] * len(img[0]))
    img.append([c] * len(img[0]))
    for row in img:
        row.insert(0, c)
        row.append(c)

def section(i, x, y):
    return [
        i[y-1][x-1], i[y-1][x], i[y-1][x+1],
        i[y][x-1], i[y][x], i[y][x+1],
        i[y+1][x-1], i[y+1][x], i[y+1][x+1],
        ]

def to_index(sec):
    s = ""
    for d in sec:
        s += "0" if d == '.' else "1"
    return int(s, base=2)

def print_image(img):
    for row in img:
        for p in row:
            print(p, end="")
        print()
    print()

def solve(inp):
    alg, img = inp
    
    # assert alg[0] == '.'
    pad(img, '.')
    pad(img, '.')
    pad(img, '.')
    for i in range(0, 50):        
        img_next = []
        for y in range(1, len(img) - 1):
            row = []
            for x in range(1, len(img[y]) - 1):
                index = to_index(section(img, x, y))
                row.append(alg[index])
            img_next.append(row)
        img = img_next
        pad(img, img[0][0])
        pad(img, img[0][0])
        pad(img, img[0][0])

        if i == 1 or i == 49:
            count = 0
            for row in img:
                for p in row:
                    if p == "#":
                        count += 1
            print(count)
    
    

    
    
    




solve(get_input())
