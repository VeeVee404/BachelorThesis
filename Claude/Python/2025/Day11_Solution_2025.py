import time
import sys
from functools import lru_cache

start = time.perf_counter()

sys.setrecursionlimit(100000)

graph = {}
with open("Day11_Input.txt") as f:
    for line in f:
        line = line.strip()
        if not line:
            continue
        node, rest = line.split(': ', 1)
        targets = rest.split()
        graph[node] = targets

@lru_cache(maxsize=None)
def count_paths(node):
    if node == 'out':
        return 1
    return sum(count_paths(t) for t in graph.get(node, []))

print(count_paths("you"))

end = time.perf_counter()
print(f"Runtime: {end - start:.6f} seconds")
