with open("Day6_Input.txt") as f:
    lines = f.read().splitlines()

# Remove truly empty lines
lines = [l for l in lines if l]

# Pad all lines to the same length
max_len = max(len(l) for l in lines)
lines = [l.ljust(max_len) for l in lines]

op_line = lines[-1]
num_lines = lines[:-1]

# A separator column is one where every line has a space
separator = [all(line[c] == ' ' for line in lines) for c in range(max_len)]

# Find contiguous non-separator column blocks (each block = one problem)
problems = []
i = 0
while i < max_len:
    if not separator[i]:
        j = i
        while j < max_len and not separator[j]:
            j += 1
        problems.append((i, j))
        i = j
    else:
        i += 1

total = 0
for start, end in problems:
    op_str = op_line[start:end].strip()
    if not op_str or op_str[0] not in '+-*':
        continue
    op = op_str[0]

    nums = []
    for row in num_lines:
        n_str = row[start:end].strip()
        if n_str:
            nums.append(int(n_str))

    if not nums:
        continue

    if op == '+':
        result = sum(nums)
    else:  # *
        result = 1
        for n in nums:
            result *= n

    total += result

print(total)
