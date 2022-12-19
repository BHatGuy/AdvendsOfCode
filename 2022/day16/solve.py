import sys

def solve(valves, time, current, open):
    if time == 0:
        return 0
    # try all options and choose the best
    # options: open currend valve if closed, or go to another valve
    flow = 0
    if current not in open:
        flow = valves[current][0] * (time - 1)
        flow += solve(valves, time - 1, current, open + [current])


    for dest in valves[current][1]:
        flow = max(flow, solve(valves, time - 1, dest, open))

    return flow


with open(sys.argv[1], "r") as f:
    lines = f.readlines()

lines = [l.strip() for l in lines]
valves = dict()
for line in lines:
    s = line.split(" ")
    name = s[1]
    rate = int(s[4][5:-1])
    tunnels = [t.replace(",", "") for t in s[9:]]
    valves[name] = (rate, tunnels)


res = solve(valves, 30, "AA", [])
print(res)
