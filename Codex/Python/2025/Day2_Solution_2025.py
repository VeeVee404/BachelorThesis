def ceil_div(a: int, b: int) -> int:
    return -(-a // b)


def range_sum(start: int, end: int) -> int:
    return (start + end) * (end - start + 1) // 2


def sum_invalid_in_range(start: int, end: int) -> int:
    total = 0
    max_digits = len(str(end))

    for half_len in range(1, max_digits // 2 + 1):
        base = 10 ** half_len + 1
        min_half = 1 if half_len == 1 else 10 ** (half_len - 1)
        max_half = 10 ** half_len - 1

        first = max(min_half, ceil_div(start, base))
        last = min(max_half, end // base)

        if first <= last:
            total += base * range_sum(first, last)

    return total


def main() -> None:
    with open("Day2_Input.txt", "r", encoding="utf-8") as file:
        content = file.read().strip()

    total = 0
    for part in content.split(","):
        if not part:
            continue
        start_text, end_text = part.split("-")
        total += sum_invalid_in_range(int(start_text), int(end_text))

    print(total)


if __name__ == "__main__":
    main()
