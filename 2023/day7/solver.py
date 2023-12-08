#!/bin/env python3
import functools
from collections import Counter


def kind_with_jokers(hand):
    result = -1
    for replacement in "123456789TJQKA":
        h = (hand[0].replace("J", replacement), 0)
        result = max(result, kind(h))
    return result


def kind(hand):
    cards = hand[0]
    count = Counter(list(cards))

    if 5 in count.values():
        return 7
    if 4 in count.values():
        return 6
    if 3 in count.values() and 2 in count.values():
        return 5
    if 3 in count.values():
        return 4
    if list(count.values()).count(2) == 2:
        return 3
    if 2 in count.values():
        return 2
    return 1


def card_to_int(card):
    if card.isdecimal():
        return int(card)
    if card == "T":
        return 10
    if card == "J":
        return 11
    if card == "Q":
        return 12
    if card == "K":
        return 13
    if card == "A":
        return 14
    raise RuntimeError


def compare_card_by_card(a, b):
    for a, b in zip(a[0], b[0]):
        if card_to_int(a) < card_to_int(b):
            return -1
        if card_to_int(a) > card_to_int(b):
            return 1
    return 0


def compare_hands(a, b):
    if kind(a) == kind(b):
        return compare_card_by_card(a, b)
    if kind(a) > kind(b):
        return 1
    if kind(a) < kind(b):
        return -1
    raise RuntimeError()

def compare_hands_with_jokers(a, b):
    if kind_with_jokers(a) == kind_with_jokers(b):
        a = (a[0].replace("J", "0"), 0)
        b = (b[0].replace("J", "0"), 0)
        return compare_card_by_card(a, b)
    if kind_with_jokers(a) > kind_with_jokers(b):
        return 1
    if kind_with_jokers(a) < kind_with_jokers(b):
        return -1
    raise RuntimeError()


with open("input.txt", "r") as f:
    data = f.read().splitlines()


hands = []
for line in data:
    cards, bid = line.split(" ")
    hands.append((cards, int(bid)))

hands.sort(key=functools.cmp_to_key(compare_hands))
print(sum([bid * rank  for rank, (_, bid) in enumerate(hands, start=1)]))

hands.sort(key=functools.cmp_to_key(compare_hands_with_jokers))
print(sum([bid * rank  for rank, (_, bid) in enumerate(hands, start=1)]))

