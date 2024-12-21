inp = open("../../../input/2024/21").read()

p1 = 0
p2 = 0


def sol(code, pos, steps, v):
    v.add((code, pos[0], pos[1]))

    if len(code) == 0:
        print(code, steps)
        return [steps]

    if num[pos[0]][pos[1]] == code[0]:
        steps.append("A")
        return sol(code[1:], pos, steps, v)

    opt = []
    for k in d:
        dy,dx = d[k]
        py,px = pos[0] + dy, pos[1] + dx

        if num[py][px] == "-1":
            continue
        if (code, py, px) in v:
            continue

        cp = steps.copy()
        cp.append(k)
        print(steps)
        print(cp, k, pos)
        for ans in sol(code, [py,px], cp.copy(), v):
            opt.append(ans)

    return opt 


for code in inp.split("\n"):
    code = code.strip()

    nr = sol(code, [4, 3], [], set())
    print(nr)

print(inp)
