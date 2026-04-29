from collections import Counter

with open("Day1_Input.txt") as f:
    left, right = zip(*(map(int, line.split()) for line in f))

counts = Counter(right)
print(sum(n * counts[n] for n in left))
