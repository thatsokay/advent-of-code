from operator import itemgetter

from part1 import marbles

if __name__ == '__main__':
    with open('input.txt') as f:
        players, last = map(int, itemgetter(0, 6)(f.readline().split()))
        print(marbles(players, last * 100))
