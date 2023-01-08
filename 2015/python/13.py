# Advent of Code 2015, day 13
# (c) aichingert

import itertools

def solve(ppl,hap):
    ans = 0

    for arr in itertools.permutations(ppl):
        cur = 0
        for i in range(len(arr)-1):
            cur += hap[(arr[i],arr[i+1])] + hap[(arr[i+1],arr[i])] 
        ans = max(ans, cur + hap[(arr[-1],arr[0])] + hap[(arr[0],arr[-1])])
    return ans

ppl = set()
hap = dict()

for l in open("../input/13").read().split('\n'):
    vls = l.split(' ')
    h = int(vls[3])
    if vls[2] == 'lose':
        h *= -1
    ppl.add(vls[0])
    hap[(vls[0],vls[-1][0:len(vls[-1])-1])] = h

print("Part 1:", solve(ppl,hap))

for p in ppl:
    hap[(p, "you")] = 0
    hap[("you", p)] = 0
ppl.add("you")

print("Part 2:", solve(ppl,hap))
