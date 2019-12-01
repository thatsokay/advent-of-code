def fuel(mass, acc=0):
    if mass <= 6:
        return acc
    new_mass = mass // 3 - 2
    return fuel(new_mass, acc + new_mass)

with open('input.txt') as f:
    lines = f.readlines()
module_masses = list(map(int, lines))
print(sum(x // 3 - 2 for x in module_masses))
print(sum(fuel(module) for module in module_masses))
