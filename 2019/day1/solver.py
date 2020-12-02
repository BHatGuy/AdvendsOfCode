def read_input():
    inp = []
    with open("input", "r") as f:
        for line in f.readlines():
            line = line.strip('\n')
            inp.append(int(line))
    return inp

def solve2():
    modules = read_input()
    total_fuel = 0
    for mod in modules:
        mass = mod
        while mass > 0:
            fuel = mass // 3 - 2
            mass = fuel 
            if fuel > 0:
                total_fuel += fuel

    return total_fuel

if __name__ == "__main__":
    print(solve2())