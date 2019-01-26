def gen_grid(serial):
    grid = {}
    for x in range(1, 301):
        for y in range(1, 301):
            rack_id = x + 10
            grid[x, y] = ((rack_id * y + serial) * rack_id) % 1000 // 100 - 5
    return grid

def square_power(grid, size):
    power = {}
    for x in range(1, 301 - size):
        for y in range(1, 301 - size):
            power[x, y] = sum([
                grid[i, j]
                for i in range(x, x + size)
                for j in range(y, y + size)
            ])
    return power

if __name__ == '__main__':
    with open('input.txt') as f:
        serial = int(f.read())
    grid = gen_grid(serial)
    print(max(
        square_power(grid, 3).items(),
        key=(lambda x: x[1])
    )[0])
