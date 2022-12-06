def parse_input():
    with open('input.txt') as f:
        input = f.read()
    return input.strip()


def part1(input):
    for i in range(len(input) - 4):
        window = set(input[i: i + 4])
        if len(window) == 4:
            return i + 4


def part2(input):
    for i in range(len(input) - 14):
        window = set(input[i: i + 14])
        if len(window) == 14:
            return i + 14


input = parse_input()
print(part1(input))
print(part2(input))
