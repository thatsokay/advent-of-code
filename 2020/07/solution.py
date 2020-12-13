def parse_input():
    with open('input.txt') as f:
        lines = f.readlines()
    return parse_bags(lines)

def part1(inputs):
    def contains_shiny_gold(colour):
        bag = inputs[colour]
        if 'shiny gold' in bag:
            return True
        for child in bag.keys():
            if contains_shiny_gold(child):
                return True
        return False

    count = 0
    for colour in inputs.keys():
        if contains_shiny_gold(colour):
            count += 1
    return count

def part2(inputs):
    def count_bags(colour):
        bag = inputs[colour]
        return sum(bag.values()) + sum(
            quantity * count_bags(child_colour)
            for child_colour, quantity
            in bag.items()
        )
    return count_bags('shiny gold')

def parse_bags(lines):
    return {colour: children for colour, children in map(parse_rule, lines)}

def parse_rule(line):
    parent_colour, rest = line.strip().split(' bags contain ')
    if rest == 'no other bags.':
        return parent_colour, {}
    children = rest[:-1].split(', ')
    return parent_colour, {
        colour: quantity
        for colour, quantity
        in map(parse_child, children)
    }

def parse_child(child):
    parts = child.split(' ')
    return ' '.join(parts[1:-1]), int(parts[0])


inputs = parse_input()

print(part1(inputs))
print(part2(inputs))
