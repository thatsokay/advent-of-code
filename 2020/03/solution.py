from functools import reduce
from operator import mul


def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return [line.strip() for line in lines]

def part1(inputs):
    return count_trees(inputs, (3, 1))

def part2(inputs):
    steps = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ]
    return reduce(mul, (count_trees(inputs, step) for step in steps))

def count_trees(slope, step):
    step_x, step_y = step
    tree_count = 0
    pos_x = step_x
    pos_y = step_y
    while pos_y < len(slope):
        if slope[pos_y][pos_x] == '#':
            tree_count += 1
        pos_x = (pos_x + step_x) % len(slope[0])
        pos_y += step_y
    return tree_count


inputs = parse_input()

print(part1(inputs))
print(part2(inputs))
