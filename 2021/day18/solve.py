import itertools
import copy

def flatten(list_of_lists):
    flat_list = list()

    def flatten_list(list_of_lists):
        for item in list_of_lists:
            if type(item) == list:
                flatten_list(item)
            else:
                flat_list.append(item)

        return flat_list

    flatten_list(list_of_lists)
    return flat_list


class Int(object):
    def __init__(self, a=0):
        self.val = a

    def __index__(self):
        return self.val

    def __str__(self):
        return str(self.val)

    def __repr__(self):
        return self.__str__()

    def __iadd__(self, a):
        if type(a) == Int:
            self.val += a.val
        else:
            self.val += a
        return self

    def __gt__(self, a):
        return self.val > a

    def __floordiv__(self, a):
        return Int(self.val // a)

    def __truediv__(self, a):
        floor = self.val // a
        d = self.val / a
        if not (floor - d == 0):
            floor += 1
        return Int(floor)


def get_input(name="input.txt"):
    lines = []
    with open(name, "r") as f:
        lines = f.readlines()

    return [parse_sfn(x)[0] for x in lines]


def parse_sfn(num):
    all = num
    if not num[0] == "[":
        digit = Int(int(num[0:1]))
        return (digit, 1)
    num = num[1:]
    e1, length = parse_sfn(num)
    num = num[length:]
    assert (num[0] == ",")
    num = num[1:]
    e2, length = parse_sfn(num)
    num = num[length:]
    assert (num[0] == "]")
    num = num[1:]
    return ([e1, e2], len(all) - len(num))


def add(a, b):
    return [a, b]


def expolde(num, all, depth=0):
    if type(num) is Int:
        return False

    if depth == 4:
        return True

    a, b = num
    res = expolde(a, all, depth + 1)
    if res:
        if depth == 3:
            f = flatten(all)
            left, right = num[0]
            prev = None
            for n in f:
                if n == left:
                    if prev is not None:
                        prev += left
                if prev == right:
                    n += right
                prev = n
            num[0] = Int(0)
        return True

    res = expolde(b, all, depth + 1)
    if res:
        if depth == 3:
            f = flatten(all)
            left, right = num[1]
            prev = None
            for n in f:
                if n == left:
                    if prev is not None:
                        prev += left
                if prev == right:
                    n += right
                prev = n
            num[1] = Int(0)

        return True

    return False


def split(num):
    a, b = num
    if type(a) == Int:
        if a > 9:
            num[0] = [a // 2, a / 2]
            return True
    else:
        res = split(a)
        if res:
            return True
    if type(b) == Int:
        if b > 9:
            num[1] = [b // 2, b / 2]
            return True
    else:
        res = split(b)
        if res:
            return True
    return False


def reduce(num):
    while True:
        if expolde(num, num):
            continue
        if split(num):
            continue
        break


def magnitude(num):

    if type(num) == Int:
        return num.val
    else:
        a, b = num
        ma = magnitude(a)
        mb = magnitude(b)
        return 3 * ma + 2 * mb


def solve1(numbers):
    acc = numbers[0]
    for num in numbers[1:]:
        acc = add(acc, num)
        reduce(acc)

    return magnitude(acc)


def solve2(numbers):
    m = -(1 << 63)
    
    for a,b in itertools.combinations(numbers, 2):
        s1 = add(copy.deepcopy(a), copy.deepcopy(b))
        s2 = add(copy.deepcopy(b), copy.deepcopy(a))
        reduce(s1)
        reduce(s2)
        mag1 = magnitude(s1)
        mag2 = magnitude(s2)
        m = max(m, mag1, mag2)
    return m

print(solve1(get_input()))
print(solve2(get_input()))
