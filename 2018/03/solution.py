from collections import Counter

def parse_input():
    with open('input.txt') as f:
        return [parse_line(line) for line in f.readlines()]

def part1(rectangles):
    return overlaps(rectangles)[1]

def part2(rectangles):
    fabric = overlaps(rectangles)[0]
    for rectangle in rectangles:
        if check_separate(rectangle, fabric):
            return rectangle[0]

def parse_line(line):
    parts = line.split()
    claim_id = parts[0][1:]
    x, y = map(int, parts[2][:-1].split(','))
    w, h = map(int, parts[3].split('x'))
    return (claim_id, (x, y), (x + w, y + h))

def overlaps(rectangles):
    fabric = Counter()
    overlap_area = 0
    for _, (x1, y1), (x2, y2) in rectangles:
        for x in range(x1, x2):
            for y in range(y1, y2):
                if fabric[x, y] == 1:
                    overlap_area += 1
                fabric[x, y] += 1
    return fabric, overlap_area

def check_separate(rectangle, fabric):
    _, (x1, y1), (x2, y2) = rectangle
    for x in range(x1, x2):
        for y in range(y1, y2):
            if fabric[x, y] != 1:
                return False
    return True


rectangles = parse_input()
print(part1(rectangles))
print(part2(rectangles))
