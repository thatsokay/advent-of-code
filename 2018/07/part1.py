from operator import itemgetter
from collections import defaultdict

def construct_graph(arcs):
    graph = defaultdict(list)
    for dependency, dependent in arcs:
        graph[dependency].append(dependent)
        graph[dependent]
    return graph

def find_order(graph):
    order = []
    while graph:
        dependents = {d for ds in graph.values() for d in ds}
        available = set(graph.keys()) - dependents
        next_step = min(available)
        order.append(next_step)
        graph.pop(next_step)
    return order

if __name__ == '__main__':
    with open('input.txt') as f:
        arcs = [itemgetter(1, 7)(line.split()) for line in f.readlines()]
        graph = construct_graph(arcs)
        print(''.join(find_order(graph)))
