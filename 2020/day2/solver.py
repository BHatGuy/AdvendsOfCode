#!/usr/bin/env python3

def read_input():
    inp = []
    with open("input", "r") as f:
        for line in f.readlines():
            line = line.strip('\n')
            amount, letter, passwd = line.split(" ")
            letter = letter.strip(":")
            mini, maxi = amount.split("-")
            mini = int(mini)
            maxi = int(maxi)
            inp.append((mini, maxi, letter, passwd))
    return inp


def solve1():
    inp = read_input()
    valid = 0
    for mini, maxi, letter, passwd in inp:
        c = passwd.count(letter)
        if c >= mini and c <= maxi:
            valid += 1
    return valid


def solve2():
    inp = read_input()
    valid = 0
    for fst, snd, letter, passwd in inp:
        c1 = passwd[fst-1]
        c2 = passwd[snd-1]
        c = c1+c2
        if c.count(letter) == 1:
            valid += 1
    return valid


if __name__ == "__main__":
    print(solve1())
    print(solve2())
