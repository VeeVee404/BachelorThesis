with open("../../../Codex/Python/2024/Day24_Input.txt") as f:
    text = f.read()

init_part, gates_part = text.strip().split("\n\n")

wires = {}
for line in init_part.splitlines():
    name, val = line.split(": ")
    wires[name] = int(val)

gates = []
for line in gates_part.splitlines():
    left, out = line.split(" -> ")
    a, op, b = left.split()
    gates.append((a, op, b, out))

pending = list(gates)
while pending:
    progress = False
    next_pending = []
    for a, op, b, out in pending:
        if a in wires and b in wires:
            va, vb = wires[a], wires[b]
            if op == "AND":
                wires[out] = va & vb
            elif op == "OR":
                wires[out] = va | vb
            elif op == "XOR":
                wires[out] = va ^ vb
            progress = True
        else:
            next_pending.append((a, op, b, out))
    pending = next_pending
    if not progress:
        break

z_wires = sorted((k for k in wires if k.startswith('z')), reverse=True)
result = int(''.join(str(wires[z]) for z in z_wires), 2)
print(result)
