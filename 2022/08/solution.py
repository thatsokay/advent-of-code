from functools import reduce
from operator import mul


def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    heights = {}
    x_len = len(lines[0].strip())
    y_len = len(lines)
    for y, line in enumerate(lines):
        for x, c in enumerate(line.strip()):
            heights[x, y] = int(c)
    return heights, x_len, y_len


def part1(input):
    return sum(1 if is_visible(coord, *input) else 0 for coord in input[0].keys())


def part2(input):
    return max(scenic_score(coord, *input) for coord in input[0].keys())


def is_visible(coord, heights, x_len, y_len):
    height = heights[coord]
    for x in range(0, coord[0]):
        if heights[x, coord[1]] >= height:
            break
    else:
        return True
    for x in range(coord[0] + 1, x_len):
        if heights[x, coord[1]] >= height:
            break
    else:
        return True
    for y in range(0, coord[1]):
        if heights[coord[0], y] >= height:
            break
    else:
        return True
    for y in range(coord[1] + 1, y_len):
        if heights[coord[0], y] >= height:
            break
    else:
        return True
    return False


def scenic_score(coord, heights, x_len, y_len):
    height = heights[coord]
    scores = [0, 0, 0, 0]
    for x in range(coord[0] - 1, -1, -1):
        scores[0] += 1
        if heights[x, coord[1]] >= height:
            break
    for x in range(coord[0] + 1, x_len):
        scores[1] += 1
        if heights[x, coord[1]] >= height:
            break
    for y in range(coord[1] - 1, -1, -1):
        scores[2] += 1
        if heights[coord[0], y] >= height:
            break
    for y in range(coord[1] + 1, y_len):
        scores[3] += 1
        if heights[coord[0], y] >= height:
            break
    return reduce(mul, scores)


input = parse_input()
print(part1(input))
print(part2(input))
