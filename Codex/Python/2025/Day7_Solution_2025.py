def main() -> None:
    with open("Day7_input.txt", "r", encoding="utf-8") as file:
        grid = [line.rstrip("\n") for line in file if line.rstrip("\n")]

    start_col = grid[0].index("S")
    active = {start_col}
    split_count = 0
    width = len(grid[0])

    for row in grid[1:]:
        next_active = set()
        for col in active:
            if row[col] == "^":
                split_count += 1
                if col > 0:
                    next_active.add(col - 1)
                if col + 1 < width:
                    next_active.add(col + 1)
            else:
                next_active.add(col)
        active = next_active
        if not active:
            break

    print(split_count)


if __name__ == "__main__":
    main()
