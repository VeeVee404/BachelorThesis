import sys
from functools import lru_cache

sys.setrecursionlimit(50000)

with open("Day7_Input.txt") as f:
    grid = [line.rstrip('\n') for line in f]

rows = len(grid)
cols = max(len(row) for row in grid)
grid = [row.ljust(cols) for row in grid]

start_col = grid[0].index('S')

@lru_cache(maxsize=None)
def count(r, c):
    if r >= rows or c < 0 or c >= cols:
        return 1
    if grid[r][c] == '^':
        return count(r, c - 1) + count(r, c + 1)
    return count(r + 1, c)

print(count(0, start_col))
