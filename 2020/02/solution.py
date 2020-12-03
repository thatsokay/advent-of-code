def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return list(map(parse_line, lines))

def part1(inputs):
    valid_count = 0
    for ((lower_bound, upper_bound), letter), password in inputs:
        letter_count = 0
        for c in password:
            if c == letter:
                letter_count += 1
        if lower_bound <= letter_count <= upper_bound:
            valid_count += 1
    return valid_count

def part2(inputs):
    valid_count = 0
    for (positions, letter), password in inputs:
        c1 = password[positions[0] - 1]
        c2 = password[positions[1] - 1]
        if (c1 == letter) ^ (c2 == letter):
            valid_count += 1
    return valid_count

def parse_line(line):
    bounds, letter, password = line.split(' ')
    bounds = tuple(map(int, bounds.split('-')))
    letter = letter[0]
    return (bounds, letter), password.strip()


inputs = parse_input()

print(part1(inputs))
print(part2(inputs))
