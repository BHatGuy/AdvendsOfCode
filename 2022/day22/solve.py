import sys

def wrap(pos, x, y):
    if pos.real >= x:
        pos = complex(0, pos.imag)
    if pos.real < 0:
        pos = complex(x - 1, pos.imag)
    if pos.imag >= y:
        pos = complex(pos.real, 0)
    if pos.imag < 0:
        pos = complex(pos.real, y - 1)
    return pos

with open(sys.argv[1], "r") as f:
    board = [l.replace("\n", "") for l in f.readlines()]

instructions = board[-1]
board = board[:-2]

length = max([len(r) for r in board])

new_board = []

for row in board:
    if len(row) < length:
        row += " " * (length - len(row))
    new_board.append(row)
board = new_board

heading = complex(1, 0)
pos = complex(board[0].index("."), 0)



current = ""
while len(instructions) > 0:
    current += instructions[0]
    instructions = instructions[1:]
    if len(instructions) > 1 and instructions[0].isnumeric() and current.isnumeric():
        continue

    if current.isalpha():
        heading = complex(heading.real, -heading.imag)
        if current == "L":
            heading *= complex(0, 1)
        else:
            heading *= complex(0, -1)
        heading = complex(heading.real, -heading.imag)

    else:
        current = int(current)
        for _ in range(current):
            new_pos = pos + heading
            new_pos = wrap(new_pos, len(board[0]), len(board))
            if board[int(new_pos.imag)][int(new_pos.real)] == "#":
                break
            elif board[int(new_pos.imag)][int(new_pos.real)] == " ":
                wall = False
                while board[int(new_pos.imag)][int(new_pos.real)] == " ":
                    new_pos = new_pos + heading
                    new_pos = wrap(new_pos, len(board[0]), len(board))
                    if board[int(new_pos.imag)][int(new_pos.real)] == "#":
                        wall = True
                        break
                if wall:
                    break
            pos = new_pos
    current = ""


facing = -1
if heading == complex(1, 0):
    facing = 0
if heading == complex(0, 1):
    facing = 1
if heading == complex(-1, 0):
    facing = 2
if heading == complex(0, -1):
    facing = 3

res = facing + 1000 * (pos.imag + 1) + 4 * (pos.real + 1)
print(res)
