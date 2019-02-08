from collections import defaultdict, deque
from functools import reduce

def parse_cavern(lines):
    cavern = []
    units = {}
    for y, line in enumerate(lines):
        for x, char in enumerate(line.strip()):
            if char != '#':
                cavern.append(x + y * 1j)
                if char != '.':
                    units[x + y * 1j] = (char, 200)
    return cavern, units

def find_distances(cavern, units, location):
    distances = defaultdict(
        lambda: float('-inf'),
        {floor: float('inf') for floor in cavern},
    )
    distances[location] = 0
    queue = deque([location])

    while queue:
        current_location = queue.popleft()
        current_distance = distances[current_location]
        for i in range(4):
            adjacent = current_location + 1j ** i
            if distances[adjacent] > current_distance + 1 and adjacent not in units:
                distances[adjacent] = current_distance + 1
                queue.append(adjacent)

    return distances

def next_step(distances, destination):
    location = destination
    while distances[location] > 1:
        location = min((
            location + 1j ** i
            for i in range(4)
            if distances[location + 1j ** i] == distances[location] - 1
        ), key=(lambda x: (x.imag, x.real)))
    return location

def unit_move(cavern, units, location):
    distances = find_distances(cavern, units, location)
    destination = min((
        loc + 1j ** i
        for (loc, (faction, _)) in units.items()
        for i in range(4)
        if faction != units[location][0] and distances[loc + 1j ** i] >= 0
    ), key=(lambda x: (distances[x], x.imag, x.real)))
    if destination != location and distances[destination] != float('inf'):
        step = next_step(distances, destination)
        units[step] = units[location]
        del units[location]
        return step
    return location

def unit_attack(cavern, units, location):
    adjacent_enemies = [
        location + 1j ** i
        for i in range(4)
        if units.get(location + 1j ** i) and
        units[location + 1j ** i][0] != units[location][0]
    ]
    if adjacent_enemies:
        target = min(adjacent_enemies, key=(lambda x: (units[x][1], x.imag, x.real)))
        hp = units[target][1]
        units[target] = units[target][0], hp - 3
        if hp <= 3:
            del units[target]

def fight(cavern, units):
    rounds = 0
    while True:
        for location in sorted(units.keys(), key=(lambda x: (x.imag, x.real))):
            if reduce(
                (lambda x, y: x if x and x[0] == y[0] else None),
                units.values(),
            ):
                return sum((unit[1] for unit in units.values())), rounds
            if units.get(location):
                move = unit_move(cavern, units, location)
                unit_attack(cavern, units, move)
        rounds += 1

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    cavern, units = parse_cavern(lines)
    hp, rounds = fight(cavern, units)
    print(hp * rounds)
