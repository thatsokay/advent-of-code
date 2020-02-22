from collections import Counter


def parse_input():
    with open('input.txt') as f:
        return f.read().strip()

def part1(pixels):
    layers = (
        Counter(pixels[i:i + 25 * 6])
        for i in range(0, len(pixels), 25 * 6)
    )
    fewest_zeros = min(layers, key=(lambda x: x['0']))
    return fewest_zeros['1'] * fewest_zeros['2']

def part2(pixels):
    image = ['2'] * 25 * 6
    for i, pixel in enumerate(pixels):
        position = i % (25 * 6)
        if image[position] == '2':
            image[position] = pixel

    return '\n'.join(
        ''.join(image[i * 25:(i + 1) * 25]).replace('0', ' ').replace('1', '#')
        for i in range(6)
    )


pixels = parse_input()
print(part1(pixels))
print(part2(pixels))
