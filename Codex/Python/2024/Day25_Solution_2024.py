import time

start = time.perf_counter()

locks = []
keys = []

with open("Day25_Input.txt", "r", encoding="utf-8") as file:
    blocks = file.read().strip().split("\n\n")

for block in blocks:
    rows = block.splitlines()
    heights = []

    for col in range(len(rows[0])):
        filled = 0
        for row in rows:
            if row[col] == "#":
                filled += 1
        heights.append(filled - 1)

    if rows[0] == "#####":
        locks.append(heights)
    else:
        keys.append(heights)

answer = 0
for lock in locks:
    for key in keys:
        fits = True
        for index in range(len(lock)):
            if lock[index] + key[index] > 5:
                fits = False
                break
        if fits:
            answer += 1

print(answer)


end = time.perf_counter()
print(f"Runtime: {end - start:.6f} seconds")
