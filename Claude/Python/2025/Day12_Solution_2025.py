lines = open("Day12_Input.txt").read().splitlines()

# Parse shapes and their cell counts
shape_cells = {}
i = 0
while i < len(lines):
    line = lines[i]
    if line and line[0].isdigit() and line.endswith(':'):
        idx = int(line[:-1])
        rows = []
        i += 1
        while i < len(lines) and lines[i].strip() and not (lines[i][0].isdigit() and lines[i].endswith(':')):
            rows.append(lines[i])
            i += 1
        shape_cells[idx] = sum(row.count('#') for row in rows)
    else:
        i += 1

# Parse regions and check if total piece area fits within grid area
cells = [shape_cells[k] for k in sorted(shape_cells)]
count = 0
for line in lines:
    if ':' in line and 'x' in line.split(':')[0]:
        dims, rest = line.split(': ')
        w, h = map(int, dims.split('x'))
        ns = list(map(int, rest.split()))
        needed = sum(n * c for n, c in zip(ns, cells))
        if needed <= w * h:
            count += 1

print(count)
