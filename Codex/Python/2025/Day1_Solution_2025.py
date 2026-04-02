def main() -> None:
    position = 50
    zero_count = 0

    with open("Day1_Input.txt", "r", encoding="utf-8") as file:
        for raw_line in file:
            line = raw_line.strip()
            if not line:
                continue

            direction = line[0]
            distance = int(line[1:]) % 100

            if direction == "L":
                position = (position - distance) % 100
            else:
                position = (position + distance) % 100

            if position == 0:
                zero_count += 1

    print(zero_count)


if __name__ == "__main__":
    main()
