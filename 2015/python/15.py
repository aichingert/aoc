# Advent of Code 2015, day 15
# (c) aichingert

def prod(a):
    ans = 1
    for i in range(len(a)-1):
        if a[i] < 0:
            a[i] = 0
        ans *= a[i]
    return ans

def solve(ing,perms,part):
    ans = 0
    for i in range(len(perms)):
        sto = [0 for _ in range(len(ing[0]))]
        for j in range(len(ing)):
            for k in range(len(ing[j])):
                sto[k] += ing[j][k] * perms[i][j]

        if part and sto[-1] != 500:
            continue
        ans = max(ans,prod(sto))
    return ans
        
ing = []
perms = []

for l in open("../input/15").read().split('\n'):
    vls = l.split(' ')
    ing.append((int(vls[2][0:-1]), int(vls[4][0:-1]), int(vls[6][0:-1]), int(vls[8][0:-1]), int(vls[10])))

for i in range(101):
    for j in range(101):
        if i + j > 100: break
        for k in range(101):
            if i + j + k > 100: break
            for l in range(101):
                if i + j + k + l > 100: break
                if i + j + k +l == 100: perms.append((i,j,k,l))

print("Part 1:", solve(ing,perms,False))
print("Part 2:", solve(ing,perms,True))
