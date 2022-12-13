import sys
import functools



def compare(a, b, prefix=""):
    if(type(a) is int and type(b) == int):
        if a < b:
            return -1
        elif a > b:
            return 1
        else:
            return 0
    elif(type(a) is list and type(b) == list):
        for aa, bb in zip(a, b):
            res = compare(aa, bb, prefix=prefix+"  ")
            if res == 1:
                return 1
            if res == -1:
                return -1
        if len(a) < len(b):
            return -1
        elif len(a) > len(b):
            return 1
        else:
            return 0
    else:
        if type(a) == int:
            res = compare([a], b, prefix=prefix+"  ")
            return res
        else:
            res = compare(a, [b], prefix=prefix+"  ")
            return res




with open(sys.argv[1], "r") as f:
    lines = f.readlines()

lines = [l.strip() for l in lines]


index = 1
sum = 0
packets = []
while lines:
    a, b = lines[0:2]
    lines = lines[3:]

    a = eval(a)
    b = eval(b)
    res = compare(a, b)
    if res == -1:
        sum += index

    index += 1
    packets.append(a)
    packets.append(b)

print(sum)

# 2
packets.append([[2]])
packets.append([[6]])

packets.sort(key=functools.cmp_to_key(compare))
decoder_key = (packets.index([[2]])+1) * (packets.index([[6]])+1)
print(decoder_key)

