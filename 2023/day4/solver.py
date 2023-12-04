#!/bin/env python3

with open("input.txt", "r") as f:
    data = f.read().splitlines()

sum = 0

cards = []

for line in data:
    _, numbers = line.split(":")
    winning_numbers, my_numbers = numbers.split("|")
    winning_numbers = [int(x) for x in winning_numbers.split(" ") if len(x) > 0]
    my_numbers = [int(x) for x in my_numbers.split(" ") if len(x) > 0]
    cards.append((winning_numbers, my_numbers))

# Part 1
for card in cards:
    winning_numbers, my_numbers = card
    winners = [x for x in my_numbers if x in winning_numbers]

    if len(winners) > 0:
        prize = 2 ** (len(winners) - 1)
        sum += prize

print(sum)

# Part 2
cards = list(enumerate(cards))
for i, card in cards:
    winning_numbers, my_numbers = card
    winners = len([x for x in my_numbers if x in winning_numbers])
    for j in range(winners):
        cards.append(cards[i + j + 1])

print(len(cards))
