from operator import itemgetter

def score_length(sequence):
    scores = '37'
    elves = (0, 1)
    while sequence not in scores[-8:]:
        scores += str(sum(map(int, itemgetter(*elves)(scores))))
        elves = tuple((i + int(scores[i]) + 1) % len(scores) for i in elves)
    return scores.find(sequence)

if __name__ == '__main__':
    with open('input.txt') as f:
        sequence = f.read().strip()
    print(score_length(sequence))
