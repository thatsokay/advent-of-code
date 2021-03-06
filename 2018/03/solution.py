from collections import Counter

def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    rectangles = [parse_line(line) for line in lines]
    return rectangles, count_layers(rectangles)

def part1(inputs):
    _, (_, overlap_area) = inputs
    return overlap_area

def part2(inputs):
    rectangles, (layers, _) = inputs
    for rectangle in rectangles:
        if is_unique(rectangle, layers):
            return rectangle[0]

def parse_line(line):
    parts = line.split()
    claim_id = parts[0][1:]
    x, y = map(int, parts[2][:-1].split(','))
    w, h = map(int, parts[3].split('x'))
    return (claim_id, (x, y), (w, h))

def count_layers(rectangles):
    layer_map = Counter()
    overlap_area = 0
    for _, (x, y), (w, h) in rectangles:
        for i in range(x, x + w):
            for j in range(y, y + h):
                if layer_map[i, j] == 1:
                    overlap_area += 1
                layer_map[i, j] += 1
    return layer_map, overlap_area

def is_unique(rectangle, layers):
    _, (x, y), (w, h) = rectangle
    for i in range(x, x + w):
        for j in range(y, y + h):
            if layers[i, j] != 1:
                return False
    return True


inputs = parse_input()
print(part1(inputs))
print(part2(inputs))
