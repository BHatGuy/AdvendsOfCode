import sys


with open(sys.argv[1], "r") as f:
    lines = f.readlines()


walls = [[[int(x) for x in p.split(",")] for p in line.strip().split(" -> ")] for line in lines]

max_y = 0

for wall in walls:
    for x, y in wall:
        max_y = max(max_y, y)

min_y = 0
min_x = 500 - max_y - 10
max_x = 500 + max_y + 10


tiles = [['.' for _ in range(0, max_x - min_x+1)] for _ in range(0, max_y - min_y + 1 +2)]
tiles[-1] = ['#' for _ in range(0, max_x - min_x+1)]
for wall in walls:
    start = wall[0]
    for end in wall[1:]:
        intervall = sorted([start[0], end[0]])
        intervall[1] +=1
        for x in range(*intervall):
            tiles[start[1] - min_y][x - min_x] = '#'
        for y in range(*sorted([start[1], end[1]])):
            tiles[y - min_y][start[0] - min_x] = '#'
        start = end

running = True
first_solve = False
count = 0
while running:
    x = 500 - min_x
    y = 0

    while True:
        old_x = x
        old_y = y
        if tiles[y+1][x] == '.':
            y = y+1
        elif tiles[y+1][x-1] == '.':
            y = y+1
            x = x-1
        elif tiles[y+1][x+1] == '.':
            y = y+1
            x = x+1
        else:
            if y== 0:
                running = False
            break

        tiles[old_y][old_x] = '.'
        tiles[y][x] = 'o'
        if y > max_y and not first_solve:
            first_solve = True
            print(count)
    count += 1

for row in tiles:
    print("".join(row))
print(count)
