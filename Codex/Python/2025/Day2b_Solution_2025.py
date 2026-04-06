from bisect import bisect_left, bisect_right


def build_invalid_ids(limit: int) -> list[int]:
    max_length = len(str(limit))
    invalid_ids: set[int] = set()

    for total_length in range(2, max_length + 1):
        for block_length in range(1, total_length // 2 + 1):
            if total_length % block_length != 0:
                continue

            repeats = total_length // block_length
            if repeats < 2:
                continue

            start = 10 ** (block_length - 1)
            end = 10 ** block_length

            for value in range(start, end):
                candidate = int(str(value) * repeats)
                if candidate <= limit:
                    invalid_ids.add(candidate)

    return sorted(invalid_ids)


def build_prefix_sums(values: list[int]) -> list[int]:
    prefix_sums = [0]
    total = 0
    for value in values:
        total += value
        prefix_sums.append(total)
    return prefix_sums


def range_sum(values: list[int], prefix_sums: list[int], start: int, end: int) -> int:
    left = bisect_left(values, start)
    right = bisect_right(values, end)
    return prefix_sums[right] - prefix_sums[left]


def main() -> None:
    with open("Day2_Input.txt", "r", encoding="utf-8") as file:
        content = file.read().strip()

    ranges: list[tuple[int, int]] = []
    max_end = 0

    for part in content.split(","):
        if not part:
            continue
        start_text, end_text = part.split("-")
        start = int(start_text)
        end = int(end_text)
        ranges.append((start, end))
        if end > max_end:
            max_end = end

    invalid_ids = build_invalid_ids(max_end)
    prefix_sums = build_prefix_sums(invalid_ids)

    total = 0
    for start, end in ranges:
        total += range_sum(invalid_ids, prefix_sums, start, end)

    print(total)


if __name__ == "__main__":
    main()
