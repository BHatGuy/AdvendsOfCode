from copy import deepcopy as copy
# Input
#  #############
#  #...........#
#  ###D#C#A#B###
#    #C#D#A#B#
#    #########

# Example
#  #############
#  #...........#
#  ###B#C#B#D###
#    #A#D#C#A#
#    #########

exits = (2, 4, 6, 8)
room_numbers = {"A": 0, "B": 1, "C": 2, "D": 3}
room_numbers_rev = {0: "A", 1: "B",  2: "C", 3: "D"}
costs = {"A": 1, "B": 10, "C": 100, "D": 1000}


def print_home(hallway, rooms):
    print(13 * "#")
    print("#" + "".join(hallway) + "#")
    print(f"###{rooms[0][0]}#{rooms[1][0]}#{rooms[2][0]}#{rooms[3][0]}###")
    print(f"  #{rooms[0][1]}#{rooms[1][1]}#{rooms[2][1]}#{rooms[3][1]}#  ")
    print("  " + 9 * "#")


def blocked(a, b):
    """a is exclusive b is inclusive"""
    s = None
    if b > a:
        s = hallway[a + 1:b + 1]
    else:
        s = hallway[b:a]
    return not all(map(lambda x: x == ".", s))
        

def possible_steps(hallway, rooms):
    steps = []
    for i, a in enumerate(hallway):
        if a == ".":
            continue
        j = room_numbers[a]
        r = rooms[j]
        if "." not in r:
            continue

        if blocked(i, exits[j]):
            continue
        
        jj = None
        if r[1] == a:
            jj = 0
        elif r[1] == ".":
            jj = 1
        if jj is not None:
            new_rooms = copy(rooms)
            hw = copy(hallway)
            new_rooms[j][jj] = a
            hw[i] = "."
            steps.append((hw, new_rooms, (jj + 1 + abs(exits[j]- i))* costs[a]))

    for j, room in enumerate(rooms):
        jj = None
        if room[0] == ".":
            if room[1] != "." and room[1] != room_numbers_rev[j]:
                jj = 1
        elif room[1] != ".":
            if not (room[1] == room_numbers_rev[j] and room[0] == room_numbers_rev[j]):
                jj = 0
        if jj is None:
            continue
        for i, a in enumerate(hallway):
            if i in exits:
                continue
            if a != ".":
                continue
            if blocked(exits[j], i):
                continue
            new_rooms = copy(rooms)
            hw = copy(hallway)
            new_rooms[j][jj] = "."
            hw[i] = room[jj]
            steps.append((hw, new_rooms, (jj + 1 + abs(exits[j]- i))* costs[room[jj]]))

    return steps

def solved(_, rooms):
    for j,room in enumerate(rooms):
        if room != [room_numbers_rev[j], room_numbers_rev[j]]:
            return False
    return  True

def solve(hallway, rooms):
    stack = [(hallway, rooms, 0)]
    cost = float("inf")
    while len(stack) > 0:
        print(len(stack), end="\r")
        # stack.sort(key=lambda x: x[2], reverse=True)
        h, r, c = stack.pop()
        if c > cost:
            continue
        steps = possible_steps(h, r)
        for s in steps:
            if solved(s[0], s[1]):
                if s[2] + c < cost:
                    cost = s[2] + c
                    print(cost)
            else:
                 if s[2] + c > cost:
                    continue
                stack.append((s[0], s[1], s[2] + c))


hallway = ["."] * 11
rooms = (["B", "A"], ["C", "D"], ["B", "C"], ["D", "A"])
# hallway = ["."] * 11
# hallway[0] = "B"
# rooms = (["A", "A"], [".", "B"], ["B", "C"], ["D", "A"])

# print_home(hallway, rooms)
# steps = possible_steps(hallway, rooms)
# for h, r, c in steps:
#     print_home(h, r)
#     print(c)

print_home(hallway, rooms)
solve(hallway, rooms)

