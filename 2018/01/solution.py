from functools import reduce
from itertools import cycle

def parse_input():
    with open('input.txt') as f:
        return [*map(int, f.readlines())]

def part1(frequencies):
    return reduce(
        lambda x, y: x + y,
        frequencies,
        0
    )

def part2(frequencies):
    cumulative = 0
    seen = set([0])
    for frequency in cycle(frequencies):
        cumulative += frequency
        if cumulative in seen:
            return cumulative
        seen.add(cumulative)

frequencies = parse_input()
print(part1(frequencies))
print(part2(frequencies))
