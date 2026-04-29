import re

OFFSET = 10000000000000

with open("Day13_Input.txt") as f:
    data = f.read()

total = 0
for block in data.strip().split("\n\n"):
    nums = list(map(int, re.findall(r'\d+', block)))
    ax, ay, bx, by, px, py = nums
    px += OFFSET
    py += OFFSET

    det = ax * by - ay * bx
    if det == 0:
        continue

    a_num = px * by - py * bx
    b_num = ax * py - ay * px

    if a_num % det == 0 and b_num % det == 0:
        a, b = a_num // det, b_num // det
        if a >= 0 and b >= 0:
            total += 3 * a + b

print(total)
