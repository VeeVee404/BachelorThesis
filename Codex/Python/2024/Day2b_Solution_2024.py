def is_safe(levels: list[int]) -> bool:
    if len(levels) < 2:
        return True

    first_diff = levels[1] - levels[0]
    if first_diff == 0 or abs(first_diff) > 3:
        return False

    increasing = first_diff > 0

    for index in range(1, len(levels)):
        diff = levels[index] - levels[index - 1]
        if diff == 0 or abs(diff) > 3:
            return False
        if increasing and diff < 0:
            return False
        if not increasing and diff > 0:
            return False

    return True


def is_safe_with_dampener(levels: list[int]) -> bool:
    if is_safe(levels):
        return True

    for index in range(len(levels)):
        if is_safe(levels[:index] + levels[index + 1 :]):
            return True

    return False


def main() -> None:
    safe_reports = 0

    with open("Day2_Input.txt", "r", encoding="utf-8") as file:
        for line in file:
            stripped = line.strip()
            if not stripped:
                continue

            levels = list(map(int, stripped.split()))
            if is_safe_with_dampener(levels):
                safe_reports += 1

    print(safe_reports)


if __name__ == "__main__":
    main()
