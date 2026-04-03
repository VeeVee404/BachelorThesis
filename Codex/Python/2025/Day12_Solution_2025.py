import time

start = time.perf_counter()

def main() -> None:
    with open("Day12_Input.txt", "r", encoding="utf-8") as file:
        lines = [line.rstrip("\n") for line in file]

    shape_areas: list[int] = []
    index = 0

    while index < len(lines):
        line = lines[index].strip()
        if not line:
            index += 1
            continue
        if "x" in line and ":" in line:
            break

        if line.endswith(":"):
            area = 0
            index += 1
            while index < len(lines) and lines[index].strip():
                area += lines[index].count("#")
                index += 1
            shape_areas.append(area)
        else:
            index += 1

    fit_count = 0

    while index < len(lines):
        line = lines[index].strip()
        index += 1
        if not line:
            continue

        size_text, counts_text = line.split(": ")
        width_text, height_text = size_text.split("x")
        width = int(width_text)
        height = int(height_text)
        counts = [int(value) for value in counts_text.split()]

        required_area = sum(count * area for count, area in zip(counts, shape_areas))
        if required_area <= width * height:
            fit_count += 1

    print(fit_count)


if __name__ == "__main__":
    start = time.perf_counter()
    main()
    end = time.perf_counter()
    print(f"Runtime: {end - start:.6f} seconds")

end = time.perf_counter()
print(f"Runtime: {end - start:.6f} seconds")