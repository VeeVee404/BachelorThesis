import time

from functools import reduce
from operator import mul


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

        values = [int(line[start:end].strip()) for line in padded[:-1]]
        operator = padded[-1][start:end].strip()

        if operator == "+":
            total += sum(values)
        else:
            total += reduce(mul, values, 1)

    print(total)


if __name__ == "__main__":
    start = time.perf_counter()
    main()
    end = time.perf_counter()
    print(f"Runtime: {end - start:.6f} seconds")
