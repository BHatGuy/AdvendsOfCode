#!/usr/bin/env python3

def read_input():
    res = []
    with open("day1_2.inp", "r") as f:
        for line in f.readlines():
            res.append(int(line))
    return res


numbers = read_input()

for i in range(0, len(numbers)):
    for j in range(0, len(numbers)):
        for k in range(0, len(numbers)):
            if numbers[i] + numbers[j] + numbers[k] == 2020:
                print(numbers[i] * numbers[j] * numbers[k])