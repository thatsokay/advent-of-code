from collections import Counter

def parse_claim(claim):
    parts = claim.split()
    claim_id = parts[0][1:]
    x, y = map(lambda x: int(x), parts[2][:-1].split(','))
    w, h = map(lambda x: int(x), parts[3].split('x'))
    return (claim_id, (x, y), (x + w, y + h))

def find_overlaps(rects):
    fabric = Counter()
    overlap_area = 0
    for _, (x1, y1), (x2, y2) in rects:
        for x in range(x1, x2):
            for y in range(y1, y2):
                if fabric[(x, y)] == 1:
                    overlap_area += 1
                fabric[(x, y)] += 1
    return fabric, overlap_area

if __name__ == '__main__':
    with open('input.txt') as f:
        rects = map(lambda x: parse_claim(x), f.readlines())
        print(find_overlaps(rects)[1])
