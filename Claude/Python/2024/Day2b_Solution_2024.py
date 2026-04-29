def is_safe(levels):
    diffs = [levels[i+1] - levels[i] for i in range(len(levels)-1)]
    return all(1 <= d <= 3 for d in diffs) or all(-3 <= d <= -1 for d in diffs)

def is_safe_dampened(levels):
    if is_safe(levels):
        return True
    return any(is_safe(levels[:i] + levels[i+1:]) for i in range(len(levels)))

with open("Day2_Input.txt") as f:
    reports = [list(map(int, line.split())) for line in f]

print(sum(is_safe_dampened(r) for r in reports))
