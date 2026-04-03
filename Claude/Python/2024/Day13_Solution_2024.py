import re

with open("../../../Codex/Python/2024/Day13_Input.txt") as f:
    text = f.read()

blocks = text.strip().split("\n\n")
total = 0

for block in blocks:
    nums = list(map(int, re.findall(r'\d+', block)))
    ax, ay, bx, by, px, py = nums

    det = ax * by - ay * bx
    if det != 0:
        a_num = px * by - py * bx
        b_num = ax * py - ay * px
        if a_num % det == 0 and b_num % det == 0:
            a, b = a_num // det, b_num // det
            if 0 <= a <= 100 and 0 <= b <= 100:
                total += 3 * a + b
    else:
        # Degenerate: brute-force up to 100 presses
        best = None
        for a in range(101):
            rx = px - ax * a
            ry = py - ay * a
            if rx >= 0 and bx != 0 and rx % bx == 0:
                b = rx // bx
                if b <= 100 and ay * a + by * b == py:
                    cost = 3 * a + b
                    if best is None or cost < best:
                        best = cost
        if best is not None:
            total += best

print(total)
