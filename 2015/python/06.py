# Advent of Code 2015, day 6
# (c) aichinger

def solve(instr, part):
    m = [[0 for _ in range(1000)] for _ in range(1000)]

    for l in instr:
        vls = l.split(" ")
        vls = vls if vls[0] != "toggle" else ["work"] + vls
        fx,fy = vls[2].split(",",1)
        tx,ty = vls[4].split(",",1)

        for y in range(int(fy),int(ty)+1):
            for x in range(int(fx),int(tx)+1):
                if vls[1] == "on":
                    if part: m[y][x] = 1
                    else: m[y][x] += 1
                elif vls[1] == "off":
                    if part: m[y][x] = 0
                    else: m[y][x] = max(0, m[y][x]-1)
                else:
                    if part: m[y][x] = 1 - m[y][x]
                    else: m[y][x] += 2
    return sum(sum(m, []))

inp = open("../input/06").read().split("\n")

print("Part 1:",solve(inp, True))
print("Part 2:",solve(inp, False))