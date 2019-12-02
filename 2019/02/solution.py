from operator import add, mul


def run_intcode(intcode, noun, verb):
    memory = list(intcode)
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
        elif intcode[ip] == 2:
            operate(mul)
        elif memory[ip] == 99:
            return memory
        else:
            raise Exception(f'Unknown opcode {memory[ip]}')
        ip += 4

with open('input.txt') as f:
    line = f.read()
intcode = list(map(int, line.split(',')))
print(run_intcode(intcode, 12, 2)[0])

for i in range(100):
    for j in range(100):
        if run_intcode(intcode, i, j)[0] == 19690720:
            print(i * 100 + j)
