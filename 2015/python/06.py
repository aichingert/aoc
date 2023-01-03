# Advent of Code 2015, day 6
# (c) aichinger

def part1(com):
    m = [[False for _ in range(1000)] for _ in range(1000)]

    for l in com:
        vls = l.split(" ")
        vls = vls if vls[0] != "toggle" else ["work"] + vls
        fx,fy = vls[2].split(",",1)
        tx,ty = vls[4].split(",",1)

        for y in range(int(fy),int(ty)+1):
            for x in range(int(fx),int(tx)+1):
                if vls[0] != "turn":
                    m[y][x] = not m[y][x]
                else:
                    if vls[1] == "on":
                        m[y][x] = True
                    else:
                        m[y][x] = False

    s = 0
    for i in range(len(m)):
        for j in range(len(m[i])):
            if m[i][j]:
                s += 1

    return s

def part2(com):
    m = [[0 for _ in range(1000)] for _ in range(1000)]

    for l in com:
        vls = l.split(" ")
        vls = vls if vls[0] != "toggle" else ["work"] + vls
        fx,fy = vls[2].split(",",1)
        tx,ty = vls[4].split(",",1)

        for y in range(int(fy),int(ty)+1):
            for x in range(int(fx),int(tx)+1):
                if vls[0] != "turn":
                    m[y][x] += 2
                else:
                    if vls[1] == "on":
                        m[y][x] += 1
                    else:
                        m[y][x] = max(0, m[y][x] - 1)
    return sum(sum(m, []))

inp = open("../input/06").read().split("\n")

print("Part 1:",part1(inp))
print("Part 2:",part2(inp))

