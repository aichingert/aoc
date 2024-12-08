p1 = set()
p2 = set()

inp = open("../../../input/2024/08").read()

G = list(filter(lambda l: l != "", inp.split("\n")))

R = len(G)
C = len(G[0])
f = {}

for i in range(R):
    for j in range(C):
        if G[i] == "":
            continue
        if G[i][j] != ".":
            if G[i][j] not in f:
                f[G[i][j]] = []
            f[G[i][j]].append([i, j])

for p in f:
    for v1 in f[p]:
        for v2 in f[p]:
            if v1 == v2:
                continue

            dy, dx = [-1 * (v1[0] - v2[0]), -1 * (v1[1] - v2[1])]
            y, x = [v1[0] + dy, v1[1] + dx]
            a, b = [v1[0] + 2 * dy, v1[1] + 2 * dx]

            if a > -1 and a < R and b > -1 and b < C:
                p1.add((a,b))

            while y > -1 and y < R and x > -1 and x < C:
                p2.add((y,x))

                y += dy
                x += dx


print("p1: ", len(p1))
print("p2: ", len(p2))
