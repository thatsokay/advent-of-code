from collections import defaultdict, Counter

def distance(coord1, coord2):
    return abs(coord1[0] - coord2[0]) + abs(coord1[1] - coord2[1])

def largest_area(coordinates):
    xs, ys = map(sorted, zip(*coordinates))
    x_bounds = (xs[0], xs[-1] + 1)
    y_bounds = (ys[0], ys[-1] + 1)
    grid = defaultdict(lambda: (None, float('inf')))
    area = Counter()
    infinites = set()
    for i, coord in enumerate(coordinates):
        for x in range(*x_bounds):
            for y in range(*y_bounds):
                curr_distance = distance(coord, (x, y))
                claim = grid[x, y]
                if curr_distance < claim[1]:
                    grid[x, y] = (i, curr_distance)
                    area[i] += 1
                    area[claim[0]] -= 1
                elif curr_distance == claim[1]:
                    grid[x, y] = (None, curr_distance)
                    area[claim[0]] -= 1
    for x in x_bounds[0], x_bounds[1] - 1:
        for y in range(*y_bounds):
            infinites.add(grid[x, y][0])
    for y in y_bounds[0], y_bounds[1] - 1:
        for x in range(*x_bounds):
            infinites.add(grid[x, y][0])
    return max(
        [(i, a) for i, a in area.items() if i not in infinites],
        key=(lambda x: x[1])
    )

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
        coordinates = [tuple(map(int, value.split(','))) for value in lines]
        print(largest_area(coordinates)[1])
