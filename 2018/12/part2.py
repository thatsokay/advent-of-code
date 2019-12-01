from part1 import gen_matcher, next_gen, sum_plants

if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()
    state = lines[0].split()[2]
    matcher = gen_matcher(lines[2:])
    # Every generation after the 101st increases the sum by 59
    start = sum_plants(state, matcher, 101)
    print(start + 59 * (50000000000 - 101))
