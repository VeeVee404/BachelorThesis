def parse_machine(block):
    a_line, b_line, prize_line = block.splitlines()

    a_part = a_line.split(": ", 1)[1]
    b_part = b_line.split(": ", 1)[1]
    prize_part = prize_line.split(": ", 1)[1]

    ax_text, ay_text = a_part.split(", ")
    bx_text, by_text = b_part.split(", ")
    px_text, py_text = prize_part.split(", ")

    ax = int(ax_text[2:])
    ay = int(ay_text[2:])
    bx = int(bx_text[2:])
    by = int(by_text[2:])
    px = int(px_text[2:])
    py = int(py_text[2:])

    return ax, ay, bx, by, px, py


with open("Day13_Input.txt", "r", encoding="utf-8") as file:
    content = file.read().strip()

machines = [parse_machine(block) for block in content.split("\n\n") if block.strip()]

total_tokens = 0

for ax, ay, bx, by, px, py in machines:
    best_cost = None

    for a_presses in range(101):
        for b_presses in range(101):
            if a_presses * ax + b_presses * bx != px:
                continue
            if a_presses * ay + b_presses * by != py:
                continue

            cost = 3 * a_presses + b_presses
            if best_cost is None or cost < best_cost:
                best_cost = cost

    if best_cost is not None:
        total_tokens += best_cost

print(total_tokens)
