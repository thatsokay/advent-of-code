from operator import add, mul


def parse_input():
    with open('input.txt') as f:
        line = f.read()
    return list(map(int, line.split(',')))

def part1(intcode):
    return run_intcode(intcode, 12, 2)

def part2(intcode):
    for noun in range(100):
        for verb in range(100):
            if run_intcode(intcode, noun, verb) == 19690720:
                return 100 * noun + verb

def run_intcode(intcode, noun, verb):
    memory = intcode.copy()
    memory[1] = noun
    memory[2] = verb
    ip = 0

    def operate(op):
        memory[memory[ip + 3]] = op(
            memory[memory[ip + 1]],
            memory[memory[ip + 2]],
        )

    while True:
        if memory[ip] == 1:
            operate(add)
        elif memory[ip] == 2:
            operate(mul)
        elif memory[ip] == 99:
            return memory[0]
        else:
            raise Exception(f'Unknown opcode {memory[ip]}')
        ip += 4


intcode = parse_input()
print(part1(intcode))
print(part2(intcode))
