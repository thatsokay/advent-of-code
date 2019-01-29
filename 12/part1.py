from operator import itemgetter

def gen_matcher(notes):
    return {
        pattern: result
        for pattern, result
        in (itemgetter(0, 2)(note.split()) for note in notes)
    }

def next_gen(state, matcher):
    return ''.join(
        matcher[state[i - 2:i + 3]]
        for i in range(2, len(state) - 2)
    )

def sum_plants(state, matcher, gens):
    for _ in range(gens):
        state = next_gen('....' + state + '....', matcher)
    offset = 2 * gens
    return sum(i - offset for i, x in enumerate(state) if x == '#')

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    state = lines[0].split()[2]
    matcher = gen_matcher(lines[2:])
    print(sum_plants(state, matcher, 20))
