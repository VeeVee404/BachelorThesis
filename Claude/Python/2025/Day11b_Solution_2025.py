import sys
from functools import lru_cache

sys.setrecursionlimit(100000)

graph = {}
with open("Day11_Input.txt") as f:
    for line in f:
        line = line.strip()
        if not line:
            continue
        node, rest = line.split(': ', 1)
        graph[node] = rest.split()

@lru_cache(maxsize=None)
def count(node, has_dac, has_fft):
    has_dac = has_dac or (node == 'dac')
    has_fft = has_fft or (node == 'fft')
    if node == 'out':
        return 1 if has_dac and has_fft else 0
    return sum(count(nb, has_dac, has_fft) for nb in graph.get(node, []))

print(count('svr', False, False))
