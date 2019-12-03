def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return list(map(int, lines))

def part1(masses):
    return sum(x // 3 - 2 for x in masses)

def part2(masses):
    return sum(fuel(module) for module in masses)

def fuel(mass, acc=0):
    if mass <= 6:
        return acc
    new_mass = mass // 3 - 2
    return fuel(new_mass, acc + new_mass)


masses = parse_input()
print(part1(masses))
print(part2(masses))
