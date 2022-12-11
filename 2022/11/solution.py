from collections import namedtuple, deque
from copy import deepcopy
from dataclasses import dataclass
from functools import reduce
from operator import add, mul
from typing import Any


@dataclass
class Monkey:
    id: int
    items: deque
    operation: Any
    test_divisible_by: int
    throw_true: int
    throw_false: int
    inspection_count: int = 0


def parse_input():
    with open('input.txt') as f:
        groups = f.read().split('\n\n')
    monkeys = []
    for group in groups:
        lines = group.split('\n')
        monkey_id = int(lines[0].split()[-1][:-1])
        items = deque(map(int, lines[1].split(': ')[-1].split(', ')))
        operation = parse_operation(lines[2])
        test_divisible_by = int(lines[3].split()[-1])
        throw_true = int(lines[4].split()[-1])
        throw_false = int(lines[5].split()[-1])

        monkeys.append(
            Monkey(
                id=monkey_id,
                items=items,
                operation=operation,
                test_divisible_by=test_divisible_by,
                throw_true=throw_true,
                throw_false=throw_false,
                inspection_count=0,
            )
        )
    return monkeys


def part1(input):
    monkeys = deepcopy(input)
    for _ in range(20):
        for monkey in monkeys:
            while len(monkey.items):
                item = monkey.operation(monkey.items.popleft()) // 3
                test_result = item % monkey.test_divisible_by == 0
                throw_to = monkey.throw_true if test_result else monkey.throw_false
                monkeys[throw_to].items.append(item)
                monkey.inspection_count += 1
    return reduce(mul, sorted((monkey.inspection_count for monkey in monkeys), reverse=True)[:2])


def part2(input):
    monkeys = deepcopy(input)
    modulo = reduce(mul, (monkey.test_divisible_by for monkey in monkeys))
    for _ in range(10000):
        for monkey in monkeys:
            while len(monkey.items):
                item = monkey.operation(monkey.items.popleft()) % modulo
                test_result = item % monkey.test_divisible_by == 0
                throw_to = monkey.throw_true if test_result else monkey.throw_false
                monkeys[throw_to].items.append(item)
                monkey.inspection_count += 1
    return reduce(mul, sorted((monkey.inspection_count for monkey in monkeys), reverse=True)[:2])


def parse_operation(line):
    operation_tokens = line.split('new = ')[-1].split()
    op = add if operation_tokens[1] == '+' else mul

    def operation(item):
        operands = [
            item if token == 'old' else int(token)
            for token
            in [operation_tokens[0], operation_tokens[2]]
        ]
        return reduce(op, operands)
    return operation


input = parse_input()
print(part1(input))
print(part2(input))
