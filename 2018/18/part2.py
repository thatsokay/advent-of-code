from part1 import parse_land, tick

def serialise(land):
    key_order = sorted(land.keys(), key=(lambda x: (x.imag, x.real)))
    return ''.join(land[key] for key in key_order)

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    land = parse_land(lines)
    tick_count = 0
    seen = {serialise(land): 0}
    for i in range(1_000_000_000):
        land = tick(land)
        tick_count += 1
        serialised = serialise(land)
        if serialised in seen:
            loop_start = seen[serialised]
            loop_duration = tick_count - loop_start
            remaining_minutes = (1_000_000_000 - loop_start) % loop_duration
            for j in range(remaining_minutes):
                land = tick(land)
            acres = tuple(land.values())
            print(acres.count('|') * acres.count('#'))
            break
        seen[serialised] = tick_count
