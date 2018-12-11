def react(polymer):
    prevChar = None
    for i, char in enumerate(polymer):
        if char == prevChar:
            return polymer[:i - 1] + polymer[i + 1:]
        else:
            prevChar = char.swapcase()
    return None

if __name__ == '__main__':
    with open('input.txt') as f:
        prev_polymer = f.read()[:-1]
        next_polymer = react(prev_polymer)
        while next_polymer is not None:
            prev_polymer = next_polymer
            next_polymer = react(prev_polymer)
        print(len(prev_polymer))
