from itertools import combinations


def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return list(map(int, lines))

def part1(inputs):
    for x, y in combinations(inputs, 2):
        if x + y == 2020:
            return x * y

def part2(inputs):
    for x, y, z in combinations(inputs, 3):
        if x + y + z == 2020:
            return x * y * z


inputs = parse_input()
print(part1(inputs))
print(part2(inputs))
