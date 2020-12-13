def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return list(map(parse_instruction, lines))

def part1(inputs):
    acc, _ = run_console(inputs)
    return acc

def part2(inputs):
    programs = (
        inputs[:i] + [('nop' if op == 'jmp' else 'jmp', arg)] + inputs[i + 1:]
        for i, (op, arg) in enumerate(inputs)
        if op == 'jmp' or op == 'nop'
    )
    for program in programs:
        acc, terminated = run_console(program)
        if terminated:
            return acc

def parse_instruction(line):
    op, arg = line.strip().split()
    return op, int(arg)

def run_console(instructions):
    acc = 0
    instruction_index = 0
    seen = set()
    while instruction_index not in seen and instruction_index < len(instructions):
        seen.add(instruction_index)
        op, arg = instructions[instruction_index]
        if op == 'acc':
            acc += arg
        elif op == 'jmp':
            instruction_index += arg
            continue
        instruction_index += 1
    return acc, instruction_index >= len(instructions)


inputs = parse_input()

print(part1(inputs))
print(part2(inputs))
