from functools import reduce
from collections import defaultdict, Counter


def get_input(name="input.txt"):
    lines = []
    with open(name, "r") as f:
        lines = f.readlines()

    return [l.strip() for l in lines]

def get_lists(inp):
    lists = defaultdict(lambda: [])
    for li in inp:
        for i, c in enumerate(li):
            i = int(i)
            lists[i].append(c)
    return lists

def solve1(inp):
    lists = get_lists(inp)
    gamma = []
    epsilon = []
    for k in lists:
        count = Counter(lists[k])
        gamma.append(count.most_common()[0][0])
        epsilon.append(count.most_common()[1][0])
    gamma = "".join(gamma)
    epsilon = "".join(epsilon)
    return int(gamma, base=2)*int(epsilon, base=2)


def solve2(inp):
    filtered = inp
    i = 0
    while len(filtered) > 1:
        lists = get_lists(filtered)
        count = Counter(lists[i])
        if count.most_common()[0][1] == count.most_common()[1][1]:
            common = "1"
        else:
            common = count.most_common()[0][0]
        filtered = list(filter(lambda x: x[i] == common, filtered))
        i += 1
    o2 = int(filtered[0], base=2)

    filtered = inp
    i = 0
    while len(filtered) > 1:
        lists = get_lists(filtered)
        count = Counter(lists[i])
        if count.most_common()[0][1] == count.most_common()[1][1]:
            common = "0"
        else:
            common = count.most_common()[1][0]
        filtered = list(filter(lambda x: x[i] == common, filtered))
        i += 1
    co2 = int(filtered[0], base=2)
    return o2 * co2


if __name__ == "__main__":
    print(solve1(get_input()))
    print(solve2(get_input()))