from functools import lru_cache


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
    def count_paths(node: str) -> int:
        if node == "out":
            return 1
        return sum(count_paths(next_node) for next_node in graph.get(node, []))

    print(count_paths("you"))


if __name__ == "__main__":
    main()
