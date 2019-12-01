from operator import itemgetter
from collections import defaultdict, deque

def marbles(players, last):
    circle = deque([0])
    scores = defaultdict(int)
    for marble in range(1, last + 1):
        if marble % 23 == 0:
            circle.rotate(7)
            scores[marble % players + 1] += marble + circle.popleft()
        else:
            circle.rotate(-2)
            circle.appendleft(marble)
    return max(scores.values())

if __name__ == '__main__':
    with open('input.txt') as f:
        players, last = map(int, itemgetter(0, 6)(f.readline().split()))
        print(marbles(players, last))
