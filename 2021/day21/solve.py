import itertools
from copy import copy
from functools import cache

def deterministic_die():
    count = 0
    while True:
        count += 1
        if count == 101:
            count = 1
        yield count


def solve1(inp):
    players = [inp[0] - 1, inp[1] - 1]
    scores = [0, 0]
    die_count = 0
    die = deterministic_die()
    while True:
        for p in range(0, 2):
            steps = sum(itertools.islice(die, 3))
            die_count += 3
            players[p] = (steps + players[p]) % 10
            scores[p] += players[p] + 1
            if scores[p] >= 1000:
                res = min(scores) * die_count
                print(res)
                return


@cache
def play(p1, p2, s1, s2, p, rolls):
    players = [p1, p2]
    scores = [s1, s2]

    if rolls == 0:
        scores[p] += players[p] + 1
        if scores[p] >= 21:
            wins = [0, 0]
            wins[p] = 1
            return wins
        else:
            return play(players[0], players[1], scores[0], scores[1], (p + 1) % 2, 3)
    else:
        wins = [0, 0]
        for steps in range(1, 4):
            ps = copy(players)
            ps[p] = (steps + ps[p]) % 10
            w = play(ps[0], ps[1], scores[0], scores[1], p, rolls - 1)
            wins[0] += w[0]
            wins[1] += w[1]            
        return wins

def solve2(inp):
    wins = play(inp[0] - 1, inp[1] - 1, 0, 0, 0, 3)       
    print(max(wins))
            


example = (4, 8)
puzzle = (6, 10)
solve1(puzzle)
solve2(puzzle)
