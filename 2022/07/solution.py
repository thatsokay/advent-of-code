from collections import defaultdict


def parse_input():
    with open('input.txt') as f:
        input = f.read()
    return input.strip().split('\n')


def part1(input):
    directory_sizes = navigate_filesystem(input)
    return sum(filter(lambda size: size <= 100000, directory_sizes.values()))


def part2(input):
    directory_sizes = navigate_filesystem(input)
    required_size = 30000000 - 70000000 + directory_sizes[('/',)]
    return min(filter(lambda size: size >= required_size, directory_sizes.values()))


def navigate_filesystem(lines):
    directory_path = ['/']
    directory_sizes = defaultdict(int)
    for line in lines:
        if line.startswith('$ cd'):
            arg = line[5:]
            if arg == '/':
                directory_path = ['/']
            elif arg == '..':
                directory_path.pop()
            else:
                directory_path.append(line[5:])
        elif line == '$ ls':
            pass
        elif line.startswith('dir'):
            pass
        else:
            for i in range(len(directory_path)):
                directory_sizes[
                    tuple(directory_path[0:i + 1])
                ] += int(line.split(' ')[0])
    return directory_sizes


input = parse_input()
print(part1(input))
print(part2(input))
