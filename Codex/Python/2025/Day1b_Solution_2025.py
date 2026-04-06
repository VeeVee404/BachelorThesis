def count_zero_hits(position: int, direction: str, distance: int) -> int:
    if direction == "L":
        first_hit = position if position != 0 else 100
    else:
        first_hit = (100 - position) if position != 0 else 100

    if distance < first_hit:
        return 0

    return 1 + (distance - first_hit) // 100


def main() -> None:
    position = 50
    zero_count = 0

    with open("Day1_Input.txt", "r", encoding="utf-8") as file:
        for raw_line in file:
            line = raw_line.strip()
            if not line:
                continue

            direction = line[0]
            distance = int(line[1:])

            zero_count += count_zero_hits(position, direction, distance)

            if direction == "L":
                position = (position - distance) % 100
            else:
                position = (position + distance) % 100

    print(zero_count)


if __name__ == "__main__":
    main()
