def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return [(line[0], line[2]) for line in lines]


def part1(input):
    score = 0
    for (opponent_play, my_play) in input:
        match my_play:
            case 'X':
                match opponent_play:
                    case 'A':
                        score += 4
                    case 'B':
                        score += 1
                    case 'C':
                        score += 7
            case 'Y':
                match opponent_play:
                    case 'A':
                        score += 8
                    case 'B':
                        score += 5
                    case 'C':
                        score += 2
            case 'Z':
                match opponent_play:
                    case 'A':
                        score += 3
                    case 'B':
                        score += 9
                    case 'C':
                        score += 6
    return score


def part2(input):
    score = 0
    for (opponent_play, my_play) in input:
        match my_play:
            case 'X':
                match opponent_play:
                    case 'A':
                        score += 3
                    case 'B':
                        score += 1
                    case 'C':
                        score += 2
            case 'Y':
                match opponent_play:
                    case 'A':
                        score += 4
                    case 'B':
                        score += 5
                    case 'C':
                        score += 6
            case 'Z':
                match opponent_play:
                    case 'A':
                        score += 8
                    case 'B':
                        score += 9
                    case 'C':
                        score += 7
    return score


input = parse_input()
print(part1(input))
print(part2(input))
