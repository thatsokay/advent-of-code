def parse_land(lines):
    return {
        x + y * 1j: value
        for y, line in enumerate(lines)
        for x, value in enumerate(line.strip())
    }

ADJACENCIES = tuple(
    (x - 1) + (y - 1) * 1j
    for x in range(3)
    for y in range(3)
    if not (x == 1 and y == 1)
)

def tick(land):
    new_land = {}
    for coord in land.keys():
        land_type = land[coord]
        adjacent_acres = tuple(
            land[coord + adjacent]
            for adjacent in ADJACENCIES
            if 0 <= coord.real + adjacent.real < 50
            and 0 <= coord.imag + adjacent.imag < 50
        )
        if land_type == '.':
            if adjacent_acres.count('|') >= 3:
                new_land[coord] = '|'
            else:
                new_land[coord] = '.'
        elif land_type == '|':
            if adjacent_acres.count('#') >= 3:
                new_land[coord] = '#'
            else:
                new_land[coord] = '|'
        elif land_type == '#':
            if '#' in adjacent_acres and '|' in adjacent_acres:
                new_land[coord] = '#'
            else:
                new_land[coord] = '.'
    return new_land

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    land = parse_land(lines)
    for i in range(10):
        land = tick(land)
    acres = tuple(land.values())
    print(acres.count('|') * acres.count('#'))
