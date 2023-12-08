#!/bin/env python3

from functools import reduce


def solve_races(times, distances):
    win_counts = []
    for time, record in zip(times, distances):
        wins = 0
        for hold in range(time):
            drive_time = time - hold
            distance = drive_time * hold
            if distance > record:
                wins += 1

        win_counts.append(wins)

    print(reduce(lambda a, b: a * b, win_counts, 1))


with open("input.txt", "r") as f:
    data = f.read().splitlines()

times = [int(x) for x in data[0].split(":")[1].strip().split(" ") if len(x) > 0]
distances = [int(x) for x in data[1].split(":")[1].strip().split(" ") if len(x) > 0]
solve_races(times, distances)

times = [int(data[0].split(":")[1].strip().replace(" ", ""))]
distances = [int(data[1].split(":")[1].strip().replace(" ", ""))]
solve_races(times, distances)
