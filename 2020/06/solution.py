from functools import reduce


def parse_input():
    with open('input.txt') as f:
        contents = f.read()
    return contents.split('\n\n')

def part1(inputs):
    return sum(len(set(group.replace('\n', ''))) for group in inputs)

def part2(inputs):
    return sum(len(group_yeses(group)) for group in inputs)


def group_yeses(group):
    person_yeses = (set(person) for person in group.split())
    return reduce(lambda a, b: a.intersection(b), person_yeses)


inputs = parse_input()

print(part1(inputs))
print(part2(inputs))
