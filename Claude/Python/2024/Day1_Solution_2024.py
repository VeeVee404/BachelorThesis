with open("../../../Codex/Python/2024/Day1_Input.txt") as f:
    left, right = zip(*(map(int, line.split()) for line in f))

print(sum(abs(a - b) for a, b in zip(sorted(left), sorted(right))))
