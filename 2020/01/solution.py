def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return list(map(int, lines))

def part1(inputs):
    for i, x in enumerate(inputs):
        for y in inputs[i + 1:]:
            if x + y == 2020:
                return x * y

def part2(inputs):
    for i, x in enumerate(inputs):
        for j, y in enumerate(inputs[i + 1:]):
            for z in inputs[j + 1:]:
                if x + y + z == 2020:
                    return x * y * z


inputs = parse_input()
print(part1(inputs))
print(part2(inputs))
