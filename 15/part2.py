from functools import reduce
from itertools import count

from part1 import parse_cavern, find_distances, next_step, unit_move

def unit_attack(cavern, units, location, damage):
    adjacent_enemies = [
        location + 1j ** i
        for i in range(4)
        if units.get(location + 1j ** i) and
        units[location + 1j ** i][0] != units[location][0]
    ]
    if adjacent_enemies:
        target = min(adjacent_enemies, key=(lambda x: (units[x][1], x.imag, x.real)))
        hp = units[target][1]
        if units[location][0] == 'E':
            units[target] = units[target][0], hp - damage
            if hp <= damage:
                del units[target]
        else:
            units[target] = units[target][0], hp - 3
            if hp <= 3:
                return False
    return True

def fight(cavern, units, damage):
    rounds = 0
    while True:
        for location in sorted(units.keys(), key=(lambda x: (x.imag, x.real))):
            if reduce(
                (lambda x, y: x if x and x[0] == y[0] else None),
                units.values(),
            ):
                return sum((unit[1] for unit in units.values())), rounds, units.popitem()[1][0]
            if units.get(location):
                move = unit_move(cavern, units, location)
                if not unit_attack(cavern, units, move, damage):
                    return None, rounds, 'G'
        rounds += 1

def winning_damage(cavern, units):
    for damage in count(3):
        result = fight(cavern, dict(units), damage)
        if result[2] == 'E':
            return result[:2]

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    cavern, units = parse_cavern(lines)
    hp, rounds = winning_damage(cavern, units)
    print(hp * rounds)
