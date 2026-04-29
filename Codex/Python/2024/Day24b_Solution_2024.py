def parse_input():
    with open("Day24_Input.txt", "r", encoding="utf-8") as file:
        _, gate_block = file.read().strip().split("\n\n")

    gates = {}
    reverse = {}

    for line in gate_block.splitlines():
        expression, output = line.split(" -> ")
        left, operation, right = expression.split()
        gates[output] = (left, operation, right)
        reverse[(operation, tuple(sorted((left, right))))] = output

    return gates, reverse


def find_gate_output(reverse, operation, left, right):
    return reverse.get((operation, tuple(sorted((left, right)))))


def main() -> None:
    _, reverse = parse_input()

    bit_count = max(
        int(wire[1:])
        for operation, inputs in reverse
        for wire in inputs
        if wire.startswith(("x", "y"))
    ) + 1

    swaps = []
    carry = find_gate_output(reverse, "AND", "x00", "y00")

    for bit in range(1, bit_count):
        x_wire = f"x{bit:02d}"
        y_wire = f"y{bit:02d}"
        z_wire = f"z{bit:02d}"

        propagate = find_gate_output(reverse, "XOR", x_wire, y_wire)
        generate = find_gate_output(reverse, "AND", x_wire, y_wire)

        sum_output = find_gate_output(reverse, "XOR", propagate, carry)
        carry_and = find_gate_output(reverse, "AND", propagate, carry)

        if sum_output == z_wire:
            carry = find_gate_output(reverse, "OR", generate, carry_and)
            continue

        if carry_and == z_wire:
            swaps.extend((sum_output, carry_and))
            carry = find_gate_output(reverse, "OR", generate, sum_output)
            continue

        if generate == z_wire:
            swaps.extend((sum_output, generate))
            carry = find_gate_output(reverse, "OR", sum_output, carry_and)
            continue

        carry_out = None
        if carry_and is not None:
            carry_out = find_gate_output(reverse, "OR", generate, carry_and)
        if carry_out == z_wire:
            swaps.extend((sum_output, carry_out))
            carry = sum_output
            continue

        alternate_sum = find_gate_output(reverse, "XOR", generate, carry)
        alternate_and = find_gate_output(reverse, "AND", generate, carry)
        if alternate_sum == z_wire and alternate_and is not None:
            swaps.extend((propagate, generate))
            carry = find_gate_output(reverse, "OR", propagate, alternate_and)
            continue

        raise RuntimeError(f"Unable to resolve swapped wires at bit {bit:02d}")

    print(",".join(sorted(swaps)))


if __name__ == "__main__":
    main()
