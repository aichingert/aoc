# Advent of Code 2015, day 9
# (c) aichingert

import itertools

dist = {}
cities = set()

def solve(dist,cities,part):
    if part: m = 99999999
    else: m = 0

    for p in itertools.permutations(cities):
        cur = 0
        for i in range(1, len(p)):
            cur += dist[p[i-1],p[i]]
        if part:
            m = min(m, cur)
        else:
            m = max(m, cur)
    return m

for l in open("../input/09").read().split('\n'):
    vls = l.split(' ')
    dist[(vls[0],vls[2])] = int(vls[4])
    dist[(vls[2],vls[0])] = int(vls[4])
    cities.add(vls[0])
    cities.add(vls[2])

print("Part 1:",solve(dist,cities,True))
print("Part 2:",solve(dist,cities,False))
