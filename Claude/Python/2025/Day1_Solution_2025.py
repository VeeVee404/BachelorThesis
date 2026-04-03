import time

start = time.perf_counter()

with open("Day1_Input.txt") as f:
    rotations = f.read().split()

pos = 50
count = 0

for rot in rotations:
    direction = rot[0]
    dist = int(rot[1:])
    if direction == 'L':
        pos = (pos - dist) % 100
    else:
        pos = (pos + dist) % 100
    if pos == 0:
        count += 1

print(count)

end = time.perf_counter()
print(f"Runtime: {end - start:.6f} seconds")
