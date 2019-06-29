from collections import Counter

def parse_reservoir(lines):
    reservoir = {}
    min_y = float('inf')
    max_y = 0
    for line in lines:
        values = line.replace('=', ', ').replace('..', ', ').split(', ')
        fixed_axis = 1 if values[0] == 'x' else 1j
        fixed_coord = int(values[1])
        range_start = int(values[3])
        range_end = int(values[4])
        if values[0] == 'x':
            min_y = min(min_y, range_start)
            max_y = max(max_y, range_end)
        else:
            min_y = min(min_y, fixed_coord)
            max_y = max(max_y, fixed_coord)
        for i in range(range_start, range_end + 1):
            reservoir[
                fixed_coord * fixed_axis +
                i * (fixed_axis * -1j).conjugate()
            ] = '#'
    return reservoir, min_y, max_y

def fill(reservoir, min_y, max_y):
    stack = [500 + max(min_y, 1) * 1j]

    while stack:
        coord = stack.pop()

        below = reservoir.get(coord + 1j) if coord.imag < max_y else '|'
        # Below cannot be +
        if below == '|':
            reservoir[coord] = '|'
        elif below == '#' or below == '~':
            left = reservoir.get(coord - 1)
            if left == '|':
                reservoir[coord] = '|'
                # Fill right
                if not reservoir.get(coord + 1):
                    stack.append(coord + 1)
            elif left == '#' or left == '+':
                # Left is # or +
                right = reservoir.get(coord + 1)
                # Right cannot be +
                if right == '|':
                    # Resolve and fill previous
                    reservoir[coord] = '|'
                elif right == '#' or right == '~':
                    # Resolve and fill previous
                    reservoir[coord] = '~'
                else:
                    # Right is None
                    reservoir[coord] = '+'
                    # Fill right then resolve current
                    stack.extend([coord, coord + 1])
            elif left == '~':
                # Right is resolved
                pass
            else:
                # Left is None
                # Fill left then fill current
                stack.extend([coord, coord - 1])
        else:
            # Below is None
            # Fill below then fill current
            stack.extend([coord, coord + 1j])

    return reservoir

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    reservoir, min_y, max_y = parse_reservoir(lines)
    fill(reservoir, min_y, max_y)
    counts = Counter(reservoir.values())
    print(counts['~'] + counts['|'])
