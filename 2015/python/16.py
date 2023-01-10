# Advent of Code 2015, day 16
# (c) aichingert

S = {"children": 3, "cats": 7, "samoyeds": 2, "pomeranians": 3, "akitas": 0, "vizslas": 0, "goldfish": 5, "trees": 3, "cars": 2, "perfumes": 1}

def part1(aunts):
    ans = ("", 0)

    for aunt in aunts:
        cur = 0
        for k in S.keys():
            if k in aunt[1] and aunt[1][k] == S[k]:
                cur += 1
        if cur > ans[1]:
            ans = (aunt[0], cur)

    return ans[0]

def part2(aunts):
    ans = ("", 0)

    for aunt in aunts:
        cur = 0
        for k in S.keys():
            if k in aunt[1]:
                if k == "cats" or k == "trees":
                    if S[k] <= aunt[1][k]:
                        cur += 1
                elif k == "pomeranians" or k == "goldfish":
                    if S[k] >= aunt[1][k]:
                        cur += 1
                else:
                    if S[k] == aunt[1][k]:
                        cur += 1

        if cur > ans[1]:
            ans = (aunt[0], cur)

    return ans[0]

s = []
for l in open("../input/16").read().split('\n'):
    sue, vls = l.split(": ", 1)
    s.append((sue.split(' ')[1], {}))

    for v in vls.split(", "):
        val,amt = v.split(": ")
        s[-1][1][val] = int(amt)

print("Part 1:", part1(s))
print("Part 2:", part2(s))
