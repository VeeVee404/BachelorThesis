from collections import deque


grid = []
with open("Day12_Input.txt", "r", encoding="utf-8") as file:
    for line in file:
        row = line.strip()
        if row:
            grid.append(row)

rows = len(grid)
cols = len(grid[0]) if rows else 0
visited = [[False] * cols for _ in range(rows)]
directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]

total_price = 0

for start_row in range(rows):
    for start_col in range(cols):
        if visited[start_row][start_col]:
            continue

        plant = grid[start_row][start_col]
        queue = deque([(start_row, start_col)])
        visited[start_row][start_col] = True
        area = 0
        perimeter = 0

        while queue:
            row, col = queue.popleft()
            area += 1

            for dr, dc in directions:
                next_row = row + dr
                next_col = col + dc

                if not (0 <= next_row < rows and 0 <= next_col < cols):
                    perimeter += 1
                    continue

                if grid[next_row][next_col] != plant:
                    perimeter += 1
                    continue

                if not visited[next_row][next_col]:
                    visited[next_row][next_col] = True
                    queue.append((next_row, next_col))

        total_price += area * perimeter

print(total_price)
