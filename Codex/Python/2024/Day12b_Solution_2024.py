def count_runs(positions: list[int]) -> int:
    positions.sort()
    runs = 0
    previous = None

    for value in positions:
        if previous is None or value != previous + 1:
            runs += 1
        previous = value

    return runs


def main() -> None:
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
            stack = [(start_row, start_col)]
            visited[start_row][start_col] = True
            area = 0

            top_edges = {}
            bottom_edges = {}
            left_edges = {}
            right_edges = {}

            while stack:
                row, col = stack.pop()
                area += 1

                for dr, dc in directions:
                    next_row = row + dr
                    next_col = col + dc

                    if 0 <= next_row < rows and 0 <= next_col < cols and grid[next_row][next_col] == plant:
                        if not visited[next_row][next_col]:
                            visited[next_row][next_col] = True
                            stack.append((next_row, next_col))
                        continue

                    if dr == -1:
                        top_edges.setdefault(row, []).append(col)
                    elif dr == 1:
                        bottom_edges.setdefault(row + 1, []).append(col)
                    elif dc == -1:
                        left_edges.setdefault(col, []).append(row)
                    else:
                        right_edges.setdefault(col + 1, []).append(row)

            sides = 0
            for edge_map in (top_edges, bottom_edges, left_edges, right_edges):
                for positions in edge_map.values():
                    sides += count_runs(positions)

            total_price += area * sides

    print(total_price)


if __name__ == "__main__":
    main()
