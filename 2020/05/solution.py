def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return set(seat_id(line.strip()) for line in lines)

def part1(inputs):
    return max(inputs)

def part2(inputs):
    seat_0 = None
    seat_1 = None
    for i in range(2 ** 10):
        seat_2 = i in inputs
        if (seat_0, seat_1, seat_2) == (True, False, True):
            return i - 1
        seat_0 = seat_1
        seat_1 = seat_2

def seat_id(seat):
    row_binary = ''.join('0' if c == 'F' else '1' for c in seat[:7])
    col_binary = ''.join('0' if c == 'L' else '1' for c in seat[7:])
    return int(row_binary, 2) * 8 + int(col_binary, 2)


inputs = parse_input()

print(part1(inputs))
print(part2(inputs))
