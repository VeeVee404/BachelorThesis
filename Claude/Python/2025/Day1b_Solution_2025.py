with open("Day1_Input.txt") as f:
    rotations = f.read().split()

def count_zeros(pos, direction, amount):
    if direction == 'L':
        first = pos if pos != 0 else 100
    else:
        r = (100 - pos) % 100
        first = r if r != 0 else 100
    if first > amount:
        return 0
    return (amount - first) // 100 + 1

pos = 50
total = 0

for rot in rotations:
    direction = rot[0]
    amount = int(rot[1:])
    total += count_zeros(pos, direction, amount)
    if direction == 'L':
        pos = (pos - amount) % 100
    else:
        pos = (pos + amount) % 100

print(total)
