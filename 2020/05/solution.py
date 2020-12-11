def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return set(seat_id(line.strip()) for line in lines)

def part1(inputs):
    return max(inputs)

def part2(inputs):
    all_seats = set(range(min(inputs), max(inputs)))
    return (all_seats - inputs).pop()

def seat_id(seat):
    seat_binary = seat.replace('F', '0').replace('B', '1').replace('L', '0').replace('R', '1')
    return int(seat_binary, 2)


inputs = parse_input()

print(part1(inputs))
print(part2(inputs))
