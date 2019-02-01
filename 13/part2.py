from part1 import parse_tracks, move_cart, turn_cart

def last_cart(tracks, carts):
    while True:
        for (x, y), (direction, turn) in sorted(carts.items()):
            if (x, y) not in carts:
                continue

            next_coord = move_cart(x, y, direction)
            if carts.get(next_coord):
                del carts[x, y]
                del carts[next_coord]
            else:
                del carts[x, y]
                carts[next_coord] = turn_cart(direction, turn, tracks[next_coord])

        if len(carts) == 1:
            return carts.popitem()[0]

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    tracks, carts = parse_tracks(lines)
    print(last_cart(tracks, carts))
