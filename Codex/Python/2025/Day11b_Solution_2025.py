from functools import lru_cache


REQUIRED_BITS = {
    "dac": 1,
    "fft": 2,
}
ALL_REQUIRED = 3


def main() -> None:
    graph: dict[str, list[str]] = {}

    with open("Day11_Input.txt", "r", encoding="utf-8") as file:
        for raw_line in file:
            line = raw_line.strip()
            if not line:
                continue

            node, targets_text = line.split(": ")
            graph[node] = targets_text.split()

    @lru_cache(maxsize=None)
    def count_paths(node: str, seen_mask: int) -> int:
        seen_mask |= REQUIRED_BITS.get(node, 0)

        if node == "out":
            return 1 if seen_mask == ALL_REQUIRED else 0

        return sum(count_paths(next_node, seen_mask) for next_node in graph.get(node, []))

    print(count_paths("svr", 0))


if __name__ == "__main__":
    main()
