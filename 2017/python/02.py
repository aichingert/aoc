# Advent of Code 2017, day 2
# (c) aichingert

def div(n):
    for i in range(len(n)):
        for j in range(len(n)):
            if i == j: continue

            if n[i] % n[j] == 0: return n[i] // n[j]
            if n[j] % n[i] == 0: return n[j] // n[i]

part_one = 0
part_two = 0

for l in open("../input/02").read().strip().split('\n'):
    n = [int(i) for i in l.split('\t')]
    part_one += max(n) - min(n)
    part_two += div(n)
    
print("Part 1: ", part_one)
print("Part 2: ", part_two)
