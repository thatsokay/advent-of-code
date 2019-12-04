from collections import Counter


def parse_input():
    with open('input.txt') as f:
        line = f.read()
    start, end = map(int, line.split('-'))
    return list(map(digits, range(start, end + 1)))

def part1(guesses):
    return len([
        guess
        for guess in filter(increasing, guesses)
        if max(Counter(guess).values()) >= 2
    ])

def part2(guesses):
    return len([
        guess
        for guess in filter(increasing, guesses)
        if 2 in Counter(guess).values()
    ])

def digits(guess):
    return [int(c) for c in str(guess)]

def increasing(digits):
    for i in range(len(digits) - 1):
        if digits[i] > digits[i + 1]:
            return False
    return True


guesses = parse_input()
print(part1(guesses))
print(part2(guesses))
