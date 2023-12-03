#!/bin/env python3
from collections import defaultdict


def check_neighbours(x, y, input):
    for dy in [-1, 0, 1]:
        for dx in [-1, 0, 1]:
            if not (0 <= x + dx < len(input[0])) or not (0 <= y + dy < len(input)):
                continue
            char = input[y + dy][x + dx]
            if not char.isdecimal() and char != ".":
                return True
    return False


def get_gear(x, y, input):
    for dy in [-1, 0, 1]:
        for dx in [-1, 0, 1]:
            if not (0 <= x + dx < len(input[0])) or not (0 <= y + dy < len(input)):
                continue
            char = input[y + dy][x + dx]
            if char == "*":
                return (x + dx, y + dy)
    return None


with open("input.txt", "r") as f:
    input = f.read().splitlines()


number = ""
has_symbol = False
sum = 0
gear_map = defaultdict(list)
gears = set()

for y, row in enumerate(input):
    for x, char in enumerate(row):
        if char.isdecimal():
            number += char
            has_symbol |= check_neighbours(x, y, input)
            gear = get_gear(x, y, input)
            if gear is not None:
                gears.add(gear)
        if (not char.isdecimal() or x == len(row) - 1) and len(number) > 0:
            if has_symbol:
                sum += int(number)
            for gear in gears:
                gear_map[gear].append(int(number))
            gears = set()
            number = ""
            has_symbol = False

print(sum)

gear_sum = 0
for numbers in gear_map.values():
    if len(numbers) == 2:
        gear_sum += numbers[0] * numbers[1]
print(gear_sum)
