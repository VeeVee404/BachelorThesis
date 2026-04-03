import time
from collections import deque

start = time.perf_counter()

with open("Day7_input.txt") as f:
    grid = [line.rstrip('\n') for line in f]

rows = len(grid)
cols = max(len(row) for row in grid)
grid = [row.ljust(cols) for row in grid]

start_col = grid[0].index('S')

visited = set()
queue = deque([(0, start_col)])
split_count = 0

while queue:
    r, c = queue.popleft()
    if r < 0 or r >= rows or c < 0 or c >= cols:
        continue
    if (r, c) in visited:
        continue
    visited.add((r, c))

    if grid[r][c] == '^':
        split_count += 1
        queue.append((r, c - 1))
        queue.append((r, c + 1))
    else:
        queue.append((r + 1, c))

print(split_count)

end = time.perf_counter()
print(f"Runtime: {end - start:.6f} seconds")
