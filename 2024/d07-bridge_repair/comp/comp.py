inp = open("../../../input/2024/07").read()

p1 = 0
p2 = 0

def sol(r, eq, c, p):
    if c > r:
        return False
    if len(eq) == 0:
        return r == c

    p2o = False
    if p:
        p2o = sol(r, eq[1:], c * (10 ** len(str(eq[0]))) + eq[0], p)
    return sol(r, eq[1:], c + eq[0], p) or sol(r, eq[1:], c * eq[0], p) or p2o

for l in inp.split("\n"):
    if l == "":
        continue

    s = l.split(": ")
    r = int(s[0])
    e = [int(n) for n in s[1].split(" ")]

    if sol(r, e[1:], e[0], False):
        p1 += r

    if sol(r, e[1:], e[0], True):
        p2 += r

print("p1: ", p1)
print("p2: ", p2)
