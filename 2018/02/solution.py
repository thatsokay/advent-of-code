from collections import Counter

def parse_input():
    with open('input.txt') as f:
        return [line.strip() for line in f.readlines()]

def part1(box_ids):
    twos = 0
    threes = 0
    for box_id in box_ids:
        two, three = has_2_and_3_of_kind(box_id)
        if two:
            twos += 1
        if three:
            threes += 1
    return twos * threes

def part2(box_ids):
    num_boxes = len(box_ids)
    for i in range(num_boxes):
        for j in range(i + 1, num_boxes):
            common = common_chars(box_ids[i], box_ids[j])
            if len(common) == len(box_ids[i]) - 1:
                return common

def has_2_and_3_of_kind(box_id):
    char_counts = Counter(box_id)
    vals = char_counts.values()
    return (2 in vals, 3 in vals)

def common_chars(box_id1, box_id2):
    return ''.join(c for c, d in zip(box_id1, box_id2) if c == d)

box_ids = parse_input()
print(part1(box_ids))
print(part2(box_ids))
