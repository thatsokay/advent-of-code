from collections import defaultdict


def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return list(map(parse_bag, lines))

def part1(inputs):
    bags = defaultdict(list)
    for colour, children in inputs:
        for child in children.keys():
            bags[child].append(colour)

    def count_ancestors(colour):
        parents = bags[colour]
        return set(parents).union(*map(count_ancestors, parents))

    return len(count_ancestors('shiny gold'))

def part2(inputs):
    bags = {colour: children for colour, children in inputs}

    def count_bags(colour):
        bag = bags[colour]
        return sum(bag.values()) + sum(
            quantity * count_bags(child_colour)
            for child_colour, quantity
            in bag.items()
        )

    return count_bags('shiny gold')

def parse_bag(line):
    parent_colour, rest = line.strip().split(' bags contain ')
    if rest == 'no other bags.':
        return parent_colour, {}
    children = rest.split(', ')
    return parent_colour, {
        colour: quantity
        for colour, quantity
        in map(parse_child, children)
    }

def parse_child(child):
    quantity, rest = child.split(' ', 1)
    colour, _ = rest.rsplit(' ', 1)
    return colour, int(quantity)


inputs = parse_input()

print(part1(inputs))
print(part2(inputs))
