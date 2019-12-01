def calibrate(deltas):
    cumulative = 0
    seen = set([0])
    while True:
        for delta in deltas:
            cumulative += delta
            if cumulative in seen:
                return cumulative
            else:
                seen.add(cumulative)

if __name__ == '__main__':
    with open('input.txt') as f:
        deltas = [int(x) for x in f.readlines()]
        print(calibrate(deltas))
