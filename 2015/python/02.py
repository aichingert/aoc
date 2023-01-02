def part1(p):
    s = 0
    for l,w,h in p:
        s += 2*l*w + 2*w*h + 2*h*l + l*w
    return s

def part2(p):
    s = 0
    for l,w,h in p:
        s += 2*l + 2*w + l*w*h
    return s

p = []

for l in open("../input/02").read().split('\n'):
    if l == '': break
    vls = [int(x) for x in l.split('x')]
    vls.sort()
    p.append((vls[0],vls[1],vls[2]))

print("Part 1:",part1(p))
print("Part 2:",part2(p))
