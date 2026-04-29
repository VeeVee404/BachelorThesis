with open("Day2_Input.txt") as f:
    data = f.read().strip()

ranges = []
for part in data.split(','):
    part = part.strip()
    if not part:
        continue
    a, b = part.split('-')
    ranges.append((int(a), int(b)))

max_val = max(b for a, b in ranges)
max_digits = len(str(max_val))

invalid = set()
for L in range(1, max_digits // 2 + 1):
    start_prefix = 10 ** (L - 1) if L > 1 else 1
    end_prefix = 10 ** L
    max_k = max_digits // L
    for prefix in range(start_prefix, end_prefix):
        s = str(prefix)
        for k in range(2, max_k + 1):
            n = int(s * k)
            if n <= max_val:
                invalid.add(n)

invalid_sorted = sorted(invalid)

total = 0
for a, b in ranges:
    for n in invalid_sorted:
        if n > b:
            break
        if n >= a:
            total += n

print(total)
