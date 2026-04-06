def read_grid() -> list[str]:
    for path in ("Day7_Input.txt", "Day7_input.txt"):
        try:
            with open(path, "r", encoding="utf-8") as file:
                return [line.rstrip("\n") for line in file if line.rstrip("\n")]
        except FileNotFoundError:
            continue
    raise FileNotFoundError("Input file not found")


def main() -> None:
    grid = read_grid()

    width = len(grid[0])
    counts = [0] * width
    counts[grid[0].index("S")] = 1

    for row in grid[1:]:
        next_counts = [0] * width

        for col, count in enumerate(counts):
            if count == 0:
                continue

            if row[col] == "^":
                if col > 0:
                    next_counts[col - 1] += count
                if col + 1 < width:
                    next_counts[col + 1] += count
            else:
                next_counts[col] += count

        counts = next_counts

    print(sum(counts))


if __name__ == "__main__":
    main()
