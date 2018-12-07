from functools import reduce

if __name__ == '__main__':
    with open('input.txt') as f:
        result = reduce(
            lambda x, y: x + int(y),
            f.readlines(),
            0
        )
        print(result)
