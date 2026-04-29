OFFSET = 10_000_000_000_000


def parse_machine(block: str) -> tuple[int, int, int, int, int, int]:
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
    px = int(px_text[2:]) + OFFSET
    py = int(py_text[2:]) + OFFSET

    return ax, ay, bx, by, px, py


def extended_gcd(a: int, b: int) -> tuple[int, int, int]:
    if b == 0:
        return abs(a), 1 if a >= 0 else -1, 0

    gcd_value, x1, y1 = extended_gcd(b, a % b)
    x = y1
    y = x1 - (a // b) * y1
    return gcd_value, x, y


def ceil_div(a: int, b: int) -> int:
    return -((-a) // b)


def solve_degenerate(ax: int, ay: int, bx: int, by: int, px: int, py: int) -> int | None:
    if ax * py != ay * px or bx * py != by * px:
        return None

    if ax != 0 or bx != 0:
        target = px
        first = ax
        second = bx
    else:
        target = py
        first = ay
        second = by

    gcd_value, x0, y0 = extended_gcd(first, second)
    if target % gcd_value != 0:
        return None

    scale = target // gcd_value
    a0 = x0 * scale
    b0 = y0 * scale

    step_a = second // gcd_value
    step_b = first // gcd_value

    lower_bound = -10**30
    upper_bound = 10**30

    if step_a > 0:
        lower_bound = max(lower_bound, ceil_div(-a0, step_a))
    elif step_a < 0:
        upper_bound = min(upper_bound, (-a0) // step_a)
    elif a0 < 0:
        return None

    if step_b > 0:
        upper_bound = min(upper_bound, b0 // step_b)
    elif step_b < 0:
        lower_bound = max(lower_bound, ceil_div(b0, step_b))
    elif b0 < 0:
        return None

    if lower_bound > upper_bound:
        return None

    slope = 3 * step_a - step_b
    t = lower_bound if slope >= 0 else upper_bound

    a_presses = a0 + step_a * t
    b_presses = b0 - step_b * t

    if a_presses < 0 or b_presses < 0:
        return None

    return 3 * a_presses + b_presses


def min_tokens(machine: tuple[int, int, int, int, int, int]) -> int | None:
    ax, ay, bx, by, px, py = machine
    determinant = ax * by - ay * bx

    if determinant != 0:
        a_numerator = px * by - py * bx
        b_numerator = ax * py - ay * px

        if a_numerator % determinant != 0 or b_numerator % determinant != 0:
            return None

        a_presses = a_numerator // determinant
        b_presses = b_numerator // determinant

        if a_presses < 0 or b_presses < 0:
            return None

        return 3 * a_presses + b_presses

    return solve_degenerate(ax, ay, bx, by, px, py)


def main() -> None:
    with open("Day13_Input.txt", "r", encoding="utf-8") as file:
        content = file.read().strip()

    machines = [parse_machine(block) for block in content.split("\n\n") if block.strip()]

    total_tokens = 0
    for machine in machines:
        tokens = min_tokens(machine)
        if tokens is not None:
            total_tokens += tokens

    print(total_tokens)


if __name__ == "__main__":
    main()
