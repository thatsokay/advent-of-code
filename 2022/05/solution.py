def parse_input():
    with open('input.txt') as f:
        input = f.read()
    (cargo, instructions) = [
        section.strip().split('\n')
        for section
        in input.split('\n\n')
    ]

    stack_count = (len(cargo[0]) + 1) // 4
    stacks = [[] for _ in range(stack_count)]
    for row in cargo[:-1]:
        for i in range(stack_count):
            crate = row[i * 4 + 1]
            if crate != ' ':
                stacks[i].append(crate)
    for stack in stacks:
        stack.reverse()

    moves = []
    for instruction in instructions:
        split = instruction.split()
        moves.append(tuple(map(int, (split[1], split[3], split[5]))))

    return stacks, moves


def part1(input):
    stacks = [stack.copy() for stack in input[0]]
    for n, source, destination in input[1]:
        for _ in range(n):
            stacks[destination - 1].append(stacks[source - 1].pop())
    return ''.join(stack[-1] for stack in stacks)


def part2(input):
    stacks = [stack.copy() for stack in input[0]]
    for n, source, destination in input[1]:
        stacks[destination - 1].extend(stacks[source - 1][-n:])
        stacks[source - 1] = stacks[source - 1][:-n]
    return ''.join(stack[-1] for stack in stacks)


input = parse_input()
print(part1(input))
print(part2(input))
