def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return {
        orbiter: orbitee
        for orbitee, orbiter in (
            line.strip().split(')')
            for line in lines
        )
    }

def part1(orbits):
    return sum(
        len(path_to(orbits, target))
        for target in orbits.keys()
    )

def part2(orbits):
    path_you = path_to(orbits, 'YOU')
    path_san = path_to(orbits, 'SAN')
    length_common = sum(
        1
        for node1, node2 in zip(path_you, path_san)
        if node1 == node2
    )
    return len(path_you) + len(path_san) - 2 * length_common

def path_to(orbits, target):
    path = []
    current_node = target
    while orbits.get(current_node):
        next_node = orbits[current_node]
        path.append(next_node)
        current_node = next_node
    return path[::-1]


input = parse_input()
print(part1(input))
print(part2(input))