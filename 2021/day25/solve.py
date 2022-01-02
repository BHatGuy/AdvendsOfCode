def get_input(name="input.txt"):
    lines = []
    with open(name, "r") as f:
        lines = f.readlines()

    return [list(x.strip()) for x in lines]


def printfloor(floor):
    for row in floor:
        for p in row:
            print(p, end="")
        print()


floor = get_input()

changes = True
count = 0

while changes:
    count += 1
    changes = False
    newfloor = [["." for _ in range(len(floor[0]))] for _ in range(len(floor))]
    for y, row in enumerate(floor):
        for x, p in enumerate(row):
            if p == ".":
                continue
            elif p == ">":
                x1 = x + 1
                if x1 >= len(row):
                    x1 = 0
                if floor[y][x1] == ".":
                    changes = True
                    newfloor[y][x1] = p
                else:
                    newfloor[y][x] = p
            elif p == "v":
                newfloor[y][x] = p
            else:
                print("ERROR")
    floor = newfloor
    newfloor = [["." for _ in range(len(floor[0]))] for _ in range(len(floor))]
    for y, row in enumerate(floor):
        for x, p in enumerate(row):
            if p == ".":
                continue
            elif p == ">":
                newfloor[y][x] = p

            elif p == "v":
                y1 = y + 1
                if y1 >= len(floor):
                    y1 = 0
                if floor[y1][x] == ".":
                    changes = True
                    newfloor[y1][x] = p
                else:
                    newfloor[y][x] = p
            else:
                print("ERROR")
    floor = newfloor

print(count)