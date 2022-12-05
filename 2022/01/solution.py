def parse_input():
    with open('input.txt') as f:
        groups = f.read().split('\n\n')
    return [[*map(int, group.split())] for group in groups]


def part1(input):
    return max(sum(group) for group in input)


def part2(input):
    return sum(sorted(sum(group) for group in input)[-3:])


input = parse_input()
print(part1(input))
print(part2(input))
