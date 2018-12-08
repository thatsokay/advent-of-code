from part1 import parse_claim, find_overlaps

def check_separate(rect, fabric):
    _, (x1, y1), (x2, y2) = rect
    for x in range(x1, x2):
        for y in range(y1, y2):
            if fabric[(x, y)] != 1:
                return False
    return True

if __name__ == '__main__':
    with open('input.txt') as f:
        rects = list(map(parse_claim, f.readlines()))
        fabric = find_overlaps(rects)[0]
        for rect in rects:
            if check_separate(rect, fabric):
                print(rect[0])
                break
