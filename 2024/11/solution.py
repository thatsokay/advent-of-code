import functools
import math
import sys


def parse_input():
    with open(sys.argv[1]) as f:
        return list(map(int, f.read().strip().split()))


@functools.cache
def expand_stone(stone, times):
    if times == 0:
        return 1
    if stone == 0:
        return expand_stone(1, times - 1)
    digit_count = math.floor(math.log10(stone)) + 1
    if digit_count % 2 == 0:
        split = 10 ** (digit_count / 2)
        return expand_stone(stone // split, times - 1) + expand_stone(
            stone % split, times - 1
        )
    return expand_stone(stone * 2024, times - 1)


def part1(input):
    return sum(expand_stone(stone, 25) for stone in input)


def part2(input):
    return sum(expand_stone(stone, 75) for stone in input)


input = parse_input()
print(part1(input))
print(part2(input))
