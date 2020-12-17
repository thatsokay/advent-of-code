from itertools import combinations, accumulate


def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return list(map(int, lines))

def part1(inputs):
    for window in (inputs[i:i + 26] for i in range(len(inputs) - 26)):
        if not valid_xmas(window):
            return window[-1]

def part2(inputs):
    target = part1(inputs)
    prefix_sum = list(accumulate(inputs))
    for upper in range(2, len(inputs)):
        for lower in range(upper - 1):
            if prefix_sum[upper] - prefix_sum[lower] == target:
                contiguous = inputs[lower:upper + 1]
                return min(contiguous) + max(contiguous)

def valid_xmas(numbers):
    for x, y in combinations(numbers[:-1], 2):
        if x + y == numbers[-1]:
            return True
    return False


inputs = parse_input()

print(part1(inputs))
print(part2(inputs))
