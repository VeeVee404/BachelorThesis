with open("../../../Codex/Python/2024/Day25_Input.txt") as f:
    text = f.read()

locks, keys = [], []

for block in text.strip().split("\n\n"):
    rows = block.splitlines()
    if rows[0] == "#####":  # lock: top filled
        heights = [sum(rows[r][c] == '#' for r in range(1, 7)) for c in range(5)]
        locks.append(heights)
    else:  # key: bottom filled
        heights = [sum(rows[r][c] == '#' for r in range(0, 6)) for c in range(5)]
        keys.append(heights)

print(sum(
    all(l + k <= 5 for l, k in zip(lock, key))
    for lock in locks
    for key in keys
))
