import sys

with open(sys.argv[1], "r") as f:
    numbers = [int(l) for l in f.readlines()]

numbers = list(enumerate(numbers))

for i in range(len(numbers)):
    index = [n[0] for n in numbers].index(i)
    elem = numbers[index]
    numbers.remove(elem)
    numbers.insert((index + elem[1]) % len(numbers), elem)

index = [n[1] for n in numbers].index(0)
res = numbers[(index + 1000)  % len(numbers)][1] + numbers[(index + 2000)  % len(numbers)][1] + numbers[(index + 3000)  % len(numbers)][1]
print(res)

# 2

with open(sys.argv[1], "r") as f:
    numbers = [int(l) * 811589153 for l in f.readlines()]


numbers = list(enumerate(numbers))

for _ in range(10):
    for i in range(len(numbers)):
        index = [n[0] for n in numbers].index(i)
        elem = numbers[index]
        numbers.remove(elem)
        numbers.insert((index + elem[1]) % len(numbers), elem)

index = [n[1] for n in numbers].index(0)
res = numbers[(index + 1000)  % len(numbers)][1] + numbers[(index + 2000)  % len(numbers)][1] + numbers[(index + 3000)  % len(numbers)][1]
print(res)
