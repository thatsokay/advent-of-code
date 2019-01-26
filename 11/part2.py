from collections import Counter

from part1 import gen_grid

def gen_sat(grid):
    sat = Counter()
    for x in range(1, 301):
        for y in range(1, 301):
            sat[x, y] = grid[x, y] + sat[x, y - 1] + sat[x - 1, y] - sat[x - 1, y - 1]
    return sat

def square_sums(sat, size):
    sums = {}
    for x in range(300 - size):
        for y in range(300 - size):
            sums[x + 1, y + 1] = sat[x + size, y + size] + sat[x, y] - sat[x + size, y] - sat[x, y + size]
    return sums

if __name__ == '__main__':
    with open('input.txt') as f:
        serial = int(f.read())
    grid = gen_grid(serial)
    sat = gen_sat(grid)
    squares = [
        ((*coord, size), power)
        for size in range(1, 301)
        for coord, power in square_sums(sat, size).items()
    ]
    print(max(squares, key=(lambda x: x[1]))[0])
