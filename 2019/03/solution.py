from collections import defaultdict, namedtuple


def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()

    directions = {
        'U': -1j,
        'D': 1j,
        'L': -1,
        'R': 1,
    }
    return tuple(
        trace_wire(
            (directions[move[0]], int(move[1:]))
            for move in line.split(',')
        )
        for line in lines
    )

def part1(wires):
    wire1, wire2 = wires
    closest = min(
        wire1.keys() & wire2.keys(),
        key=lambda x: abs(x.real) + abs(x.imag),
    )
    return int(abs(closest.real) + abs(closest.imag))

def part2(wires):
    wire1, wire2 = wires
    return min(
        wire1[intersection] + wire2[intersection]
        for intersection in wire1.keys() & wire2.keys()
    )

def trace_wire(wire):
    visited = defaultdict(lambda: float('inf'))
    position = 0
    steps = 0
    for direction, distance in wire:
        for _ in range(distance):
            position += direction
            steps += 1
            visited[position] = min(steps, visited[position])
    return visited


wires = parse_input()
print(part1(wires))
print(part2(wires))
