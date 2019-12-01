def react(polymer):
    reacted = []
    unit = polymer and polymer[0] or ''
    for next_unit in polymer[1:]:
        if unit == next_unit.swapcase():
            unit = reacted and reacted.pop() or ''
        else:
            reacted.append(unit)
            unit = next_unit
    reacted.append(unit)
    return ''.join(reacted)

if __name__ == '__main__':
    with open('input.txt') as f:
        polymer = f.read().strip()
    print(len(react(polymer)))
