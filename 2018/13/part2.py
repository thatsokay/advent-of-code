from part1 import parse_tracks, turn_cart

def last_cart(tracks, carts):
    while True:
        for coord, (direction, turn) in sorted(carts.items(), key=(lambda x: (x[0].imag, x[0].real))):
            if coord not in carts:
                continue

            next_coord = coord + direction
            if carts.get(next_coord):
                del carts[coord]
                del carts[next_coord]
            else:
                del carts[coord]
                carts[next_coord] = turn_cart(direction, turn, tracks[next_coord])

        if len(carts) == 1:
            return carts.popitem()[0]

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    tracks, carts = parse_tracks(lines)
    print(last_cart(tracks, carts))
