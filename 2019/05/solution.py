def parse_input():
    with open('input.txt') as f:
        line = f.read()
    return list(map(int, line.split(',')))

def part1(intcode):
    return run_intcode(intcode, 1)

def part2(intcode):
    return run_intcode(intcode, 5)

def run_intcode(intcode, first_input):
    memory = intcode.copy()
    ip = 0
    output = []

    def eval_param(param_mode, value):
        if param_mode == '0':
            return memory[value]
        elif param_mode == '1':
            return value
        return None

    def eval_params(param_modes, values):
        return (
            eval_param(mode, val)
            for mode, val in zip(param_modes, memory[ip + 1:ip + 3])
        )

    while True:
        instruction = str(memory[ip])
        opcode = int(instruction[-2:])
        param_modes = instruction[:-2][::-1].ljust(3, '0')

        if opcode == 1:
            a, b = eval_params(param_modes, memory[ip + 1: ip + 3])
            memory[memory[ip + 3]] = a + b
            ip += 4
        elif opcode == 2:
            a, b = eval_params(param_modes, memory[ip + 1: ip + 3])
            memory[memory[ip + 3]] = a * b
            ip += 4
        elif opcode == 3:
            memory[memory[ip + 1]] = first_input
            ip += 2
        elif opcode == 4:
            output.append(eval_param(param_modes[0], memory[ip + 1]))
            ip += 2
        elif opcode == 5:
            a, b = eval_params(param_modes, memory[ip + 1: ip + 3])
            if a:
                ip = b
            else:
                ip += 3
        elif opcode == 6:
            a, b = eval_params(param_modes, memory[ip + 1: ip + 3])
            if not a:
                ip = b
            else:
                ip += 3
        elif opcode == 7:
            a, b = eval_params(param_modes, memory[ip + 1: ip + 3])
            memory[memory[ip + 3]] = 1 if a < b else 0
            ip += 4
        elif opcode == 8:
            a, b = eval_params(param_modes, memory[ip + 1: ip + 3])
            memory[memory[ip + 3]] = 1 if a == b else 0
            ip += 4
        elif opcode == 99:
            return output[-1]
        else:
            raise Exception(f'Unknown opcode {memory[ip]}')


intcode = parse_input()
print(part1(intcode))
print(part2(intcode))
