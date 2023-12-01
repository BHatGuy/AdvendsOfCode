#!/usr/bin/env python3
import re

numbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

with open("input.txt", "r") as f:
    data = f.readlines()

sum = 0
for l in data:
    digits = re.sub("[a-z]|\n", "", l)
    num = int(digits[0] + digits[-1])
    sum += num

print(sum)


sum = 0
for l in data:
    l = l.replace("\n","")

    digits = []
    for i in range(len(l)):
        if l[i].isdecimal():
            digits.append(l[i])

        for n, s in enumerate(numbers):
            if l[i:].startswith(s):
                digits.append(str(n))

    num = int(digits[0] + digits[-1])

    sum += num

print(sum)
