#!/bin/env python3
lowest = 138241
highest = 674034

def has_double_digit(n: int):
    last = None
    for c in str(n):
        if last == c:
            return True
        last = c
    return False

def has_double_digit_exact(n: int):
    last = -1
    last2 = -2
    last3 = -3
    for c in str(n):
        if last3 != last2 and last2 == last and last != c:
            return True
        last3 = last2
        last2 = last
        last = c
    if last3 != last2 and last == last2:
        return True
    return False

def never_decrease(n: int):
    last = -1
    for c in str(n):
        c = int(c)
        if last > c:
            return False
        last = c
    return True

def solve1():
    count = 0
    for n in range(lowest, highest):
        if has_double_digit(n) and never_decrease(n):
            count += 1
    return count


def solve2():
    count = 0
    for n in range(lowest, highest):
        if has_double_digit_exact(n) and never_decrease(n):
            count += 1
    return count


if __name__ == "__main__":
    print(solve1())
    print(solve2())
