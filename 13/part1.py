DIRECTIONS = ('<', '^', '>', 'v')

def parse_tracks(lines):
    tracks = {}
    carts = {}
    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            if char in r'-|\/+':
                tracks[x, y] = char
            elif char in '<^>v':
                tracks[x, y] = char
                carts[x, y] = DIRECTIONS.index(char), 0
    return tracks, carts

def move_cart(x, y, direction):
    return (
        x + ((direction + 1) % 2) * (-1)**(direction // 2 + 1),
        y + (direction % 2) * (-1)**(direction // 2 + 1),
    )

def turn_cart(direction, turn, track):
    if track == '/':
        next_direction = (direction + (-1)**(direction + 1)) % 4
        next_turn = turn
    elif track == '\\':
        next_direction = (direction + (-1)**(direction)) % 4
        next_turn = turn
    elif track == '+':
        next_direction = (direction + turn - 1) % 4
        next_turn = (turn + 1) % 3
    else:
        next_direction = direction
        next_turn = turn
    return next_direction, next_turn

def first_crash(tracks, carts):
    while True:
        for (x, y), (direction, turn) in sorted(carts.items()):
            next_coord = move_cart(x, y, direction)
            if carts.get(next_coord):
                return next_coord
            del carts[x, y]
            carts[next_coord] = turn_cart(direction, turn, tracks[next_coord])

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    tracks, carts = parse_tracks(lines)
    print(first_crash(tracks, carts))
