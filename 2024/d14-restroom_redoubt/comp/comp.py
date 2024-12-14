from collections import defaultdict
import time

inp = open("../../../input/2024/14").read()

p1 = 0

rs = defaultdict(int)

li = []

R = 103
C = 101

for l in inp.split("\n"):
    if l == "":
        continue

    p, v = l.split(" ")
    p = p[2:].split(",")
    v = v[2:].split(",")
    p = [int(p[0]), int(p[1])]
    v = [int(v[0]), int(v[1])]

    li.append([p, v])
    px, py = (p[0] + v[0] * 100) % C, (p[1] + v[1] * 100) % R
    rs[(px, py)] += 1

seg = [0, 0, 0, 0]

for i in range(0, R // 2, 1):
    for j in range(0, C // 2, 1):
        if (j, i) in rs:
            seg[0] += rs[(j, i)]
    for j in range(C // 2 + 1, C, 1):
        if (j, i) in rs:
            seg[1] += rs[(j, i)]

for i in range(R // 2 + 1, R, 1):
    for j in range(0, C // 2, 1):
        if (j, i) in rs:
            seg[2] += rs[(j, i)]
    for j in range(C // 2 + 1, C, 1):
        if (j, i) in rs:
            seg[3] += rs[(j, i)]

print("p1: ", seg[0] * seg[1] * seg[2] * seg[3])

for t in range(50000):
    m = set()
    for i in range(len(li)):
        p, v = li[i]
        m.add(((p[0] + v[0] * t) % C, (p[1] + v[1] * t) % R))

    cnt = 0

    for i in range(R // 4, 3 * R // 4, 1):
        for j in range(C // 4, 3 * C // 4, 1):
            cnt += (j, i) in m

    if cnt > len(m) / 2:
        for i in range(R):
            for j in range(C):
                if (j, i) in m:
                    print("#", end="")
                else:
                    print(".", end="")
            print()
        print("p2: ", t)
        time.sleep(2)
        print()
        print()

