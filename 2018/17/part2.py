from collections import Counter

from part1 import parse_reservoir, fill

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    reservoir, min_y, max_y = parse_reservoir(lines)
    fill(reservoir, min_y, max_y)
    counts = Counter(reservoir.values())
    print(counts['~'])
