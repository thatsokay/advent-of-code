from part1 import gen_sat, square_sums

if __name__ == '__main__':
    with open('input.txt') as f:
        serial = int(f.read())
    sat = gen_sat(serial)
    squares = (
        data
        for size in range(1, 301)
        for data in square_sums(sat, size)
    )
    print(max(squares, key=(lambda x: x[1]))[0])
