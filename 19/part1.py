ops = {
    'addr': lambda a, b, c: lambda reg: {**reg, c: reg[a] + reg[b]},
    'addi': lambda a, b, c: lambda reg: {**reg, c: reg[a] + b},
    'mulr': lambda a, b, c: lambda reg: {**reg, c: reg[a] * reg[b]},
    'muli': lambda a, b, c: lambda reg: {**reg, c: reg[a] * b},
    'banr': lambda a, b, c: lambda reg: {**reg, c: reg[a] & reg[b]},
    'bani': lambda a, b, c: lambda reg: {**reg, c: reg[a] & b},
    'borr': lambda a, b, c: lambda reg: {**reg, c: reg[a] | reg[b]},
    'bori': lambda a, b, c: lambda reg: {**reg, c: reg[a] | b},
    'setr': lambda a, b, c: lambda reg: {**reg, c: reg[a]},
    'seti': lambda a, b, c: lambda reg: {**reg, c: a},
    'gtir': lambda a, b, c: lambda reg: {**reg, c: int(a > reg[b])},
    'gtri': lambda a, b, c: lambda reg: {**reg, c: int(reg[a] > b)},
    'gtrr': lambda a, b, c: lambda reg: {**reg, c: int(reg[a] > reg[b])},
    'eqir': lambda a, b, c: lambda reg: {**reg, c: int(a == reg[b])},
    'eqri': lambda a, b, c: lambda reg: {**reg, c: int(reg[a] == b)},
    'eqrr': lambda a, b, c: lambda reg: {**reg, c: int(reg[a] == reg[b])},
}

def parse_instruction(line):
    """
    Takes a line of input and returns a corresponding function that performs the
    operation described.
    """
    opcode, a, b, c = line.split(' ')
    return ops[opcode](*map(int, (a, b, c)))

def run(instructions, register, ip):
    while register[ip] < len(instructions):
        register = instructions[register[ip]](register)
        register[ip] += 1
    return register

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    ip = int(lines[0][4:])
    program = list(map(parse_instruction, lines[1:]))
    print(run(program, {i: 0 for i in range(6)}, ip))
