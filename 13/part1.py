def parse_tracks(lines):
    DIRECTIONS = '>v<^'
    tracks = {}
    carts = {}
    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            if char in r'-|\/+':
                tracks[x + y * 1j] = char
            elif char in DIRECTIONS:
                tracks[x + y * 1j] = char
                carts[x + y * 1j] = 1j ** DIRECTIONS.index(char), 0
    return tracks, carts

def turn_cart(direction, turn, track):
    if track == '/':
        next_direction = direction * 1j if direction.imag else direction * -1j
        next_turn = turn
    elif track == '\\':
        next_direction = direction * 1j if direction.real else direction * -1j
        next_turn = turn
    elif track == '+':
        next_direction = direction * 1j ** (turn - 1)
        next_turn = (turn + 1) % 3
    else:
        next_direction = direction
        next_turn = turn
    return next_direction, next_turn

def first_crash(tracks, carts):
    while True:
        for coord, (direction, turn) in sorted(carts.items(), key=(lambda x: (x[0].imag, x[0].real))):
            next_coord = coord + direction
            if carts.get(next_coord):
                return next_coord
            del carts[coord]
            carts[next_coord] = turn_cart(direction, turn, tracks[next_coord])

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    tracks, carts = parse_tracks(lines)
    print(first_crash(tracks, carts))
