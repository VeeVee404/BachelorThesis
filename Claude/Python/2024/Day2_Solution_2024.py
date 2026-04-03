def is_safe(levels):
    diffs = [levels[i+1] - levels[i] for i in range(len(levels)-1)]
    return all(1 <= d <= 3 for d in diffs) or all(-3 <= d <= -1 for d in diffs)

with open("../../../Codex/Python/2024/Day2_Input.txt") as f:
    reports = [list(map(int, line.split())) for line in f]

print(sum(is_safe(r) for r in reports))
