#!/bin/env python3
import string


# parsing like this is probably unsave as hell!
def read_input():
    batch = []
    with open('input', 'r') as f:
        for pp in f.read().split("\n\n"):
            pp = pp.replace('\n', ' ')
            ppd = {}
            for kv in pp.split(' '):
                if len(kv) == 0:
                    continue
                k, v = kv.split(':')
                ppd[k] = v
            batch.append(ppd)
    return batch


req_fields = ('byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid')


def contains_req_fields(pp):
    return all([f in pp for f in req_fields])


def valid(pp):
    if not contains_req_fields(pp):
        return False

    if not 1920 <= int(pp['byr']) <= 2002:
        return False

    if not 2010 <= int(pp['iyr']) <= 2020:
        return False

    if not 2020 <= int(pp['eyr']) <= 2030:
        return False

    if pp['hgt'].endswith('cm'):
        h = int(pp['hgt'].replace('cm', ''))
        if not 150 <= h <= 193:
            return False
    elif pp['hgt'].endswith('in'):
        h = int(pp['hgt'].replace('in', ''))
        if not 59 <= h <= 76:
            return False
    else:
        return False

    if pp['hcl'].startswith("#"):
        col = pp['hcl'].replace('#', '')
        if not all(c in string.hexdigits for c in col):
            return False
    else:
        return False

    if pp['ecl'] not in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'):
        return False
    
    if not (len(pp['pid']) == 9 and all(c in string.digits for c in pp['pid'])):
        return False


    return True


def solve1():
    batch = read_input()

    return len(list(filter(contains_req_fields, batch)))


def solve2():
    batch = read_input()
    return len(list(filter(valid, batch)))


if __name__ == "__main__":
    print(solve1())
    print(solve2())
