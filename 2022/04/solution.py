def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return [[*map(int, line.replace(',', '-').split('-'))] for line in lines]


def part1(input):
    return sum(
        1 if (
            section_ids[0] <= section_ids[2] and section_ids[1] >= section_ids[3]
        ) or (
            section_ids[0] >= section_ids[2] and section_ids[1] <= section_ids[3]
        ) else 0
        for section_ids
        in input
    )


def part2(input):
    return sum(
        1 if (
            section_ids[0] >= section_ids[2] or section_ids[1] >= section_ids[2]
        ) and (
            section_ids[0] <= section_ids[3] or section_ids[1] <= section_ids[3]
        ) else 0
        for section_ids
        in input
    )


input = parse_input()
print(part1(input))
print(part2(input))
