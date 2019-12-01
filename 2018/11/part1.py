from collections import Counter

def cell_power(serial, x, y):
    return (((x + 10) * y + serial) * (x + 10)) % 1000 // 100 - 5

def gen_sat(serial):
    sat = Counter()
    for x in range(1, 301):
        for y in range(1, 301):
            sat[x, y] = cell_power(serial, x, y) + sat[x, y - 1] + sat[x - 1, y] - sat[x - 1, y - 1]
    return sat

def square_sums(sat, size):
    return (
        (
            (x + 1, y + 1, size),
            sat[x + size, y + size] + sat[x, y] - sat[x + size, y] - sat[x, y + size]
        )
        for x in range(300 - size)
        for y in range(300 - size)
    )

if __name__ == '__main__':
    with open('input.txt') as f:
        serial = int(f.read())
    sat = gen_sat(serial)
    print(max(
        square_sums(sat, 3),
        key=(lambda x: x[1])
    )[0][:-1])
