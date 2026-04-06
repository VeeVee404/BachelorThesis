def parse_problem(lines: list[str], start: int, end: int) -> list[int]:
    values: list[int] = []

    for col in range(end - 1, start - 1, -1):
        digits = []
        for row in range(len(lines) - 1):
            char = lines[row][col]
            if char != " ":
                digits.append(char)

        if digits:
            values.append(int("".join(digits)))

    return values


def main() -> None:
    with open("Day6_Input.txt", "r", encoding="utf-8") as file:
        lines = file.read().splitlines()

    width = max(len(line) for line in lines)
    padded = [line.ljust(width) for line in lines]

    separators = [all(line[col] == " " for line in padded) for col in range(width)]

    total = 0
    col = 0
    while col < width:
        while col < width and separators[col]:
            col += 1
        if col >= width:
            break

        start = col
        while col < width and not separators[col]:
            col += 1
        end = col

        values = parse_problem(padded, start, end)
        operator = padded[-1][start:end].strip()

        if operator == "+":
            total += sum(values)
        else:
            product = 1
            for value in values:
                product *= value
            total += product

    print(total)


if __name__ == "__main__":
    main()
