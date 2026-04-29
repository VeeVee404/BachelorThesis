import re
from collections import defaultdict

with open("Day24_Input.txt") as f:
    data = f.read()

parts = data.strip().split("\n\n")
gates = {}
for line in parts[1].splitlines():
    m = re.match(r'(\w+) (AND|OR|XOR) (\w+) -> (\w+)', line)
    a, op, b, out = m.groups()
    gates[out] = (op, a, b)

max_z = max(w for w in gates if w.startswith('z'))

# Map each wire to the operations that consume it
wire_consumers = defaultdict(list)
for out, (op, a, b) in gates.items():
    wire_consumers[a].append(op)
    wire_consumers[b].append(op)

wrong = set()

for out, (op, a, b) in gates.items():
    # Rule 1: z-outputs (except the final carry) must come from XOR gates
    if out.startswith('z') and out != max_z and op != 'XOR':
        wrong.add(out)

    # Rule 2: XOR gates whose inputs are NOT direct x/y wires must output to z
    if op == 'XOR' and not (a[0] in 'xy' and b[0] in 'xy'):
        if not out.startswith('z'):
            wrong.add(out)

    # Rule 3: XOR gates with direct x_i/y_i inputs (bit > 0) must NOT output z
    # (they produce the half-sum, not the final sum bit)
    if op == 'XOR' and a[0] in 'xy' and b[0] in 'xy' and a[1:] != '00':
        if out.startswith('z'):
            wrong.add(out)

    # Rule 4: AND gates (except x00 AND y00) must feed only into OR gates
    # (their output is a partial carry, which is combined via OR)
    if op == 'AND' and {a, b} != {'x00', 'y00'}:
        for consumer_op in wire_consumers[out]:
            if consumer_op != 'OR':
                wrong.add(out)
                break

    # Rule 5: XOR with direct x/y inputs (bit > 0) must feed into XOR and AND,
    # never into OR (the half-sum is not a carry signal)
    if op == 'XOR' and a[0] in 'xy' and b[0] in 'xy' and a[1:] != '00':
        for consumer_op in wire_consumers[out]:
            if consumer_op == 'OR':
                wrong.add(out)
                break

print(','.join(sorted(wrong)))
