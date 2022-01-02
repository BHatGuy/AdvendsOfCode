def get_input(name="input.txt"):
    lines = []
    with open(name, "r") as f:
        lines = f.readlines()
    res = []
    for l in lines:
        inst = l.strip().split(" ")
        while len(inst) < 3:
            inst.append(None)
        res.append(inst)

    return res


def exec(prog, input):
    regfile = {"w": 0, "x": 0, "y": 0, "z": 0}

    def getval(b):
        if b.isalpha():
            val = regfile[b]
        else:
            val = int(b)            
        return val

    for inst in prog:
        op, a, b = inst
        if op == "inp":
            regfile[a] = input.pop()
        elif op == "add":
            val = getval(b)
            regfile[a] += val
        elif op == "mul":
            val = getval(b)
            regfile[a] *= val
        elif op == "div":
            val = getval(b)
            regfile[a] //= val
        elif op == "mod":
            val = getval(b)
            regfile[a] = regfile[a] % val
        elif op == "eql":
            val = getval(b)
            regfile[a] = 1 if val == regfile[a] else 0
        else:
            print(inst)
    return regfile["z"]


prog = get_input()
num = [9] * 14
# num = [-9, -9, -11, -10, -13, -14, -12, -4, 0, -7, -10, -5, -12, -6]
print(exec(prog, num))