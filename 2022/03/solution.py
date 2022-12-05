from functools import reduce


def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return lines


def part1(input):
    sum = 0
    for line in input:
        half_length = len(line) // 2
        compartment_1 = set(list(line[:half_length]))
        compartment_2 = set(list(line[half_length:]))
        intersection = compartment_1.intersection(compartment_2).pop()
        sum += item_priority(intersection)
    return sum


def part2(input):
    sum = 0
    for i in range(0, len(input), 3):
        rucksacks = [set(list(line)) for line in input[i: i + 3]]
        intersection = reduce(
            lambda acc, rucksack: acc.intersection(rucksack),
            rucksacks,
        ).pop()
        sum += item_priority(intersection)
    return sum


def item_priority(item):
    if item.islower():
        return ord(item) - ord('a') + 1
    else:
        return ord(item) - ord('A') + 27


input = parse_input()
print(part1(input))
print(part2(input))
