left_values = []
right_values = []

with open("Day1_Input.txt", "r", encoding="utf-8") as file:
    for line in file:
        stripped = line.strip()
        if not stripped:
            continue
        left, right = map(int, stripped.split())
        left_values.append(left)
        right_values.append(right)

left_values.sort()
right_values.sort()

answer = 0
for left, right in zip(left_values, right_values):
    answer += abs(left - right)

print(answer)
