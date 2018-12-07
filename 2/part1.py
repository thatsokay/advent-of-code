from functools import reduce
from collections import Counter

def checksum(box_ids):
    counts = reduce(
        lambda x, y: x + Counter(has_count([2, 3], y)),
        box_ids,
        Counter(),
    )
    return counts[2] * counts[3]

def has_count(counts, box_id):
    result = {count: None for count in counts}
    min_count = min(counts)
    chars_left = box_id
    curr_len = len(box_id)
    prev_len = None
    while curr_len >= min_count:
        chars_left = chars_left.replace(chars_left[0], '')
        prev_len = curr_len
        curr_len = len(chars_left)
        count = prev_len - curr_len
        if count in counts:
            result[count] = True
        if None not in result.values():
            return result
    for key, val in result.items():
        if val is None:
            result[key] = False
    return result

if __name__ == '__main__':
    with open('input.txt') as f:
        print(checksum(f.readlines()))
