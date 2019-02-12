from string import ascii_lowercase

from part1 import react

if __name__ == '__main__':
    with open('input.txt') as f:
        polymer = f.read().strip()
    results = (
        len(react(polymer.replace(letter, '').replace(letter.upper(), '')))
        for letter in ascii_lowercase
    )
    print(min(results))
