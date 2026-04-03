from collections import deque

with open("../../../Codex/Python/2024/Day12_Input.txt") as f:
    grid = [line.rstrip('\n') for line in f]

rows, cols = len(grid), len(grid[0])
visited = [[False] * cols for _ in range(rows)]
total = 0

for sr in range(rows):
    for sc in range(cols):
        if visited[sr][sc]:
            continue
        plant = grid[sr][sc]
        queue = deque([(sr, sc)])
        visited[sr][sc] = True
        area = 0
        perimeter = 0
        while queue:
            r, c = queue.popleft()
            area += 1
            for dr, dc in [(-1,0),(1,0),(0,-1),(0,1)]:
                nr, nc = r+dr, c+dc
                if 0 <= nr < rows and 0 <= nc < cols and grid[nr][nc] == plant:
                    if not visited[nr][nc]:
                        visited[nr][nc] = True
                        queue.append((nr, nc))
                else:
                    perimeter += 1
        total += area * perimeter

print(total)
