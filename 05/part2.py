from part1 import react
import string

if __name__ == '__main__':
    with open('input.txt') as f:
        polymer = f.read()[:-1]
        results = {}
        for letter in string.ascii_lowercase:
            prev_polymer = polymer.replace(letter, '').replace(letter.upper(), '')
            next_polymer = react(prev_polymer)
            while next_polymer is not None:
                prev_polymer = next_polymer
                next_polymer = react(prev_polymer)
            results[letter] = len(prev_polymer)
        optimal = min(results.items(), key=(lambda x: x[1]))
        print(optimal[1])
