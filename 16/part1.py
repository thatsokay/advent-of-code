ops = {
    'addr': lambda reg, a, b, c: {**reg, c: reg[a] + reg[b]},
    'addi': lambda reg, a, b, c: {**reg, c: reg[a] + b},
    'mulr': lambda reg, a, b, c: {**reg, c: reg[a] * reg[b]},
    'muli': lambda reg, a, b, c: {**reg, c: reg[a] * b},
    'banr': lambda reg, a, b, c: {**reg, c: reg[a] & reg[b]},
    'bani': lambda reg, a, b, c: {**reg, c: reg[a] & b},
    'borr': lambda reg, a, b, c: {**reg, c: reg[a] | reg[b]},
    'bori': lambda reg, a, b, c: {**reg, c: reg[a] | b},
    'setr': lambda reg, a, b, c: {**reg, c: reg[a]},
    'seti': lambda reg, a, b, c: {**reg, c: a},
    'gtir': lambda reg, a, b, c: {**reg, c: int(a > reg[b])},
    'gtri': lambda reg, a, b, c: {**reg, c: int(reg[a] > b)},
    'gtrr': lambda reg, a, b, c: {**reg, c: int(reg[a] > reg[b])},
    'eqir': lambda reg, a, b, c: {**reg, c: int(a == reg[b])},
    'eqri': lambda reg, a, b, c: {**reg, c: int(reg[a] == b)},
    'eqrr': lambda reg, a, b, c: {**reg, c: int(reg[a] == reg[b])},
}

def parse_sample(sample):
    before, instruction, after = sample.strip().split('\n')
    before = dict(enumerate(map(int, before[9:-1].split(', '))))
    after = dict(enumerate(map(int, after[9:-1].split(', '))))
    instruction = [int(x) for x in instruction.split()]
    return instruction, before, after

def possible_ops(instruction, before, after):
    return {name for name, op in ops.items() if op(before, *instruction[1:]) == after}

def count_threes(samples):
    return len([
        True
        for sample in samples
        if len(possible_ops(*parse_sample(sample))) >= 3
    ])

if __name__ == '__main__':
    with open('input.txt') as f:
        samples = f.read().split('\n\n\n')[0].split('\n\n')
    print(count_threes(samples))
