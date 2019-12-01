from operator import itemgetter

def recipe_scores(offset):
    scores = [3, 7]
    elves = (0, 1)
    while len(scores) < offset + 10:
        scores.extend(map(int, str(sum(itemgetter(*elves)(scores)))))
        elves = tuple((i + scores[i] + 1) % len(scores) for i in elves)
    return ''.join(map(str, scores[offset:offset + 10]))

if __name__ == '__main__':
    with open('input.txt') as f:
        offset = int(f.read())
    print(recipe_scores(offset))
