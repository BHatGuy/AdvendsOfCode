#!/bin/env python3


with open("input.txt", "r") as f:
    input = f.read().splitlines()

sum1 = 0
sum2 = 0

for l in input:
    id, rest = l.split(":")
    id = int(id.replace("Game ", ""))

    valid = True
    maxes = dict()
    maxes["red"] = 0
    maxes["green"] = 0
    maxes["blue"] = 0
    for draw in rest.split(";"):
        for color in draw.split(","):
            count, color = color.strip().split(" ")
            count = int(count)
            maxes[color] = max(count, maxes[color])
            if (
                (color == "red" and count > 12)
                or (color == "green" and count > 13)
                or (color == "blue" and count > 14)
            ):
                valid = False

    sum2 += maxes["red"] * maxes["blue"] * maxes["green"]
    if valid:
        sum1 += id
                

print(sum1)
print(sum2)
