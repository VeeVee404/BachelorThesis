from collections import deque

with open("Day12_Input.txt") as f:
    grid = [line.rstrip('\n') for line in f]

rows, cols = len(grid), len(grid[0])
visited = [[False] * cols for _ in range(rows)]

def in_region(r, c, ch):
    return 0 <= r < rows and 0 <= c < cols and grid[r][c] == ch

def count_corners(r, c, ch):
    # Number of corners == number of sides for a region
    corners = 0
    # Check each of the 4 corner positions (pairs of orthogonal dirs + diagonal)
    for dr, dc in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
        # orthogonal neighbors: (dr,dc) and rotated 90 degrees (dc,-dr)
        n1 = in_region(r + dr, c + dc, ch)
        n2 = in_region(r + dc, c - dr, ch)
        diag = in_region(r + dr + dc, c + dc - dr, ch)
        # convex corner: both orthogonal neighbors outside
        if not n1 and not n2:
            corners += 1
        # concave corner: both orthogonal neighbors inside but diagonal outside
        elif n1 and n2 and not diag:
            corners += 1
    return corners

total = 0
for sr in range(rows):
    for sc in range(cols):
        if visited[sr][sc]:
            continue
        ch = grid[sr][sc]
        queue = deque([(sr, sc)])
        visited[sr][sc] = True
        area = 0
        sides = 0
        while queue:
            r, c = queue.popleft()
            area += 1
            sides += count_corners(r, c, ch)
            for dr, dc in [(-1,0),(1,0),(0,-1),(0,1)]:
                nr, nc = r+dr, c+dc
                if 0 <= nr < rows and 0 <= nc < cols and not visited[nr][nc] and grid[nr][nc] == ch:
                    visited[nr][nc] = True
                    queue.append((nr, nc))
        total += area * sides

print(total)
