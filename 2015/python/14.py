# Advent of Code day 2015, day 14
# (c) aichingert

T = 2503

def dist(deer, sec):
    t = deer[2] + deer[3]
    c = sec // t
    r = sec % t
    return c * deer[1] * deer[2] + deer[1] * min(r,deer[2])

def part1(deers):
    ans = 0
    for deer in deers:
        ans = max(ans, dist(deer,T))
    return ans

def part2(deers):
    ans = [0 for _ in range(len(deers))]

    for sec in range(1,T+1):
        m = 0
        inc = 0
        for (idx,deer) in enumerate(deers):
            d = dist(deer, sec)
            if d >= m:
                m = d
                inc = idx
        ans[inc] += 1
    return max(ans)

deers = []

for l in open("../input/14").read().split('\n'):
    vls = l.split(' ')
    deers.append((vls[0],int(vls[3]),int(vls[6]),int(vls[13])))

print("Part 1:", part1(deers))
print("Part 2:", part2(deers))
