from collections import Counter
from part1 import distance

def safe_region(coordinates):
    xs, ys = map(sorted, zip(*coordinates))
    x_bounds = (xs[0], xs[-1] + 1)
    y_bounds = (ys[0], ys[-1] + 1)
    safe_area = 0
    for x in range(*x_bounds):
        for y in range(*y_bounds):
            distances = 0
            for coord in coordinates:
                distances += distance((x, y), coord)
            if distances < 10000:
                safe_area += 1
    return safe_area

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
        coordinates = [tuple(map(int, value.split(','))) for value in lines]
        print(safe_region(coordinates))
