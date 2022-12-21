import sys

def read_monkeys():
    monkeys = dict()
    with open(sys.argv[1], "r") as f:
        for line in f.readlines():
            name, rest = line.strip().split(": ")
            if rest.isnumeric():
                monkeys[name] = int(rest)
            else:
                job = rest.split(" ")
                monkeys[name] = job
    return monkeys

def calc(monkeys, target):
    job = monkeys[target]
    if type(job) is int:
        return job

    op1 = calc(monkeys, job[0])
    op2 = calc(monkeys, job[2])
    if job[1] == "+":
        return op1 + op2
    if job[1] == "-":
        return op1 - op2
    if job[1] == "*":
        return op1 * op2
    if job[1] == "/":
        return op1 // op2
    print("Unknown operation")


def solve(expression, value):
    if len(expression) == 0:
        return value
    if type(expression[1]) is int:
        if expression[0] == "+":
            value = value - expression[1]
        if expression[0] == "-":
            value = -value + expression[1]
        if expression[0] == "*":
            value = value // expression[1]
        if expression[0] == "/":
            value = expression[1] / value
        return solve(expression[2], value)
    else:
        if expression[0] == "+":
            value = value - expression[2]
        if expression[0] == "-":
            value = value + expression[2]
        if expression[0] == "*":
            value = value // expression[2]
        if expression[0] == "/":
            value = value * expression[2]
        return solve(expression[1], value)

def calc_human(monkeys, target):
    if target == "humn":
        return []

    job = monkeys[target]
    if type(job) is int:
        return job

    op1 = calc_human(monkeys, job[0])
    op2 = calc_human(monkeys, job[2])
    if target == "root":
        print(f"{op1}={op2}")
        return solve(op1, op2)
    if type(op1) is list or type(op2) is list:
        return [job[1], op1, op2]
    else:
        if job[1] == "+":
            return op1 + op2
        if job[1] == "-":
            return op1 - op2
        if job[1] == "*":
            return op1 * op2
        if job[1] == "/":
            return op1 // op2
        print("Unknown operation")



monkeys = read_monkeys()
res = calc(monkeys, "root")
print(res)
res = calc_human(monkeys, "root")
print(res)
