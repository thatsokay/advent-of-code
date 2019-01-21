from operator import itemgetter
from collections import defaultdict, deque
from re import split

def find_bounds(instance):
    min_x = min_y = 1000000
    max_x = max_y = -1000000
    for x, y in instance:
        if x < min_x:
            min_x = x
        elif x > max_x:
            max_x = x
        if y < min_y:
            min_y = y
        elif y > max_y:
            max_y = y
    return min_x, min_y, max_x, max_y

def find_instance(lights):
    prev_size = 1000000000000
    delta = -1
    i = 0
    while delta < 0:
        instance = {
            (x + i * dx, y + i * dy)
            for (x, y, dx, dy) in lights
        }
        bounds = find_bounds(instance)
        size = (bounds[2] - bounds[0]) * (bounds[3] - bounds[1])
        delta = size - prev_size
        prev_size = size
        i += 1
    i -= 2
    instance = {(x + i * dx, y + i * dy) for (x, y, dx, dy) in lights}
    return instance, bounds, i

def draw(instance, bounds):
    x1, y1, x2, y2 = bounds
    for y in range(y1, y2 + 1):
        for x in range(x1, x2 + 1):
            if (x, y) in instance:
                print('#', end='')
            else:
                print('.', end='')
        print()

if __name__ == '__main__':
    with open('input.txt') as f:
        lights = [
            tuple(map(int, itemgetter(1, 2, 4, 5)(split('[<>,]', line))))
            for line in f.readlines()
        ]
    instance, bounds, time = find_instance(lights)
    draw(instance, bounds)
    print(time)
