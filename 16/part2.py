from collections import defaultdict

from part1 import ops, parse_sample, possible_ops

def find_opcodes(samples):
    opcodes = defaultdict(lambda: set(ops.keys()))
    for instruction, before, after in map(parse_sample, samples):
        opcodes[instruction[0]].intersection_update(possible_ops(instruction, before, after))
        if len(opcodes[instruction[0]]) == 1:
            for i in range(16):
                if i != instruction[0]:
                    opcodes[i].difference_update(opcodes[instruction[0]])
    return {opcode: ops[names.pop()] for opcode, names in opcodes.items()}

def run_program(opcodes, instructions):
    register = dict(enumerate((0, 0, 0, 0)))
    for opcode, a, b, c in ([int(x) for x in instr.split()] for instr in instructions):
        register = opcodes[opcode](register, a, b, c)
    return register

if __name__ == '__main__':
    with open('input.txt') as f:
        samples, program = f.read().split('\n\n\n')
    opcodes = find_opcodes(samples.strip().split('\n\n'))
    print(run_program(opcodes, program.strip().split('\n')))
