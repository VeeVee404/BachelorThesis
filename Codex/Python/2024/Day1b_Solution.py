def main() -> None:
    left_values = []
    right_counts = {}

    with open("Day1_Input.txt", "r", encoding="utf-8") as file:
        for line in file:
            stripped = line.strip()
            if not stripped:
                continue

            left_value, right_value = map(int, stripped.split())
            left_values.append(left_value)
            right_counts[right_value] = right_counts.get(right_value, 0) + 1

    similarity_score = sum(value * right_counts.get(value, 0) for value in left_values)
    print(similarity_score)


if __name__ == "__main__":
    main()
