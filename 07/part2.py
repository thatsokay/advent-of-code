from operator import itemgetter
from collections import defaultdict
from string import ascii_uppercase

def construct_graph(arcs):
    graph = defaultdict(lambda: {'dependencies': [], 'dependents': []})
    for dependency, dependent in arcs:
        graph[dependency]['dependents'].append(dependent)
        graph[dependent]['dependencies'].append(dependency)
    return graph

def time_taken(graph):
    finish_times = {}
    while graph:
        dependents = {d for ds in graph.values() for d in ds['dependents']}
        available = set(graph.keys()) - dependents
        next_steps = sorted(available)[:5]
        for step in next_steps:
            start_time = max([finish_times[d] for d in graph[step]['dependencies']] or [0])
            duration = ascii_uppercase.find(step) + 61
            finish_times[step] = start_time + duration
            graph.pop(step)
    return max(finish_times.items(), key=(lambda x: x[1]))[1]

if __name__ == '__main__':
    with open('input.txt') as f:
        arcs = [itemgetter(1, 7)(line.split()) for line in f.readlines()]
        graph = construct_graph(arcs)
        print(time_taken(graph))
