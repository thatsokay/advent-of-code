from string import ascii_lowercase

from part1 import react

if __name__ == '__main__':
    with open('input.txt') as f:
        polymer = f.read().strip()
    results = []
    for letter in ascii_lowercase:
        improved_polymer = polymer.replace(letter, '').replace(letter.upper(), '')
        results.append(len(react(improved_polymer)))
    print(min(results))
