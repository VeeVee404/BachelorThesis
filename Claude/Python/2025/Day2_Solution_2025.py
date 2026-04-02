with open("Day2_Input.txt") as f:
    data = f.read().strip()

ranges = []
for part in data.split(','):
    part = part.strip()
    if not part:
        continue
    a, b = part.split('-')
    ranges.append((int(a), int(b)))

# Generate all "doubled" numbers: digits repeated twice, no leading zeros
# prefix length 1..5 covers up to 10-digit numbers (max in input ~9.5e9)
doubled = []
for length in range(1, 6):
    start = 10 ** (length - 1)
    end = 10 ** length
    for prefix in range(start, end):
        s = str(prefix)
        n = int(s + s)
        doubled.append(n)

doubled.sort()

total = 0
for a, b in ranges:
    for n in doubled:
        if n > b:
            break
        if n >= a:
            total += n

print(total)
