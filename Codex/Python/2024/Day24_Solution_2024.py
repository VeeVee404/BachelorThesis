import time

start = time.perf_counter()

values = {}
gates = {}

with open("Day24_Input.txt", "r", encoding="utf-8") as file:
    initial_block, gate_block = file.read().strip().split("\n\n")

for line in initial_block.splitlines():
    wire, value = line.split(": ")
    values[wire] = int(value)

for line in gate_block.splitlines():
    expression, output = line.split(" -> ")
    left, operation, right = expression.split()
    gates[output] = (left, operation, right)


def evaluate(wire):
    if wire in values:
        return values[wire]

    left, operation, right = gates[wire]
    left_value = evaluate(left)
    right_value = evaluate(right)

    if operation == "AND":
        result = left_value & right_value
    elif operation == "OR":
        result = left_value | right_value
    else:
        result = left_value ^ right_value

    values[wire] = result
    return result


z_wires = sorted((wire for wire in gates if wire.startswith("z")), reverse=True)
answer = 0
for wire in z_wires:
    answer = (answer << 1) | evaluate(wire)

print(answer)


end = time.perf_counter()
print(f"Runtime: {end - start:.6f} seconds")
