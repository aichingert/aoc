# Advent of Code 2015, day 18
# (c) aichingert

def sur(p, g):
    ans = 0
    for i in range(-1,2):
        for j in range(-1,2):
            if i == 0 == j:
                continue
            if not (p[0]+i < 0 or p[0]+i >= len(g) or p[1]+j < 0 or p[1]+j >= len(g[p[0]])) and g[p[0]+i][p[1]+j] == '#':
                ans += 1
    return ans

def s_c(g):
    g[0][0] = '#'
    g[0][len(grid[0])-1] = '#'
    g[len(g)-1][0] = '#'
    g[len(g)-1][len(g[0])-1] = '#'
    return g

def solve(g, part):
    for _ in range(100):
        n = [['.' for _ in range(len(g[0]))] for _ in range(len(g))]
        if part: s_c(n)

        ans = 0
        for i in range(len(g)):
            for j in range(len(g[i])):
                if part and (i,j) in [(0,0), (0,len(g[i])-1), (len(g)-1,0), (len(g)-1,len(g[i])-1)]:
                        continue
                nb = sur([i,j], g)
                if g[i][j] == '#':
                    if nb == 2 or nb == 3:
                        n[i][j] = '#'
                    else:
                        n[i][j] = '.'
                else:
                    if g[i][j] == '.' and nb == 3:
                        n[i][j] = '#'
                    else:
                        n[i][j] = '.'
        g = n
    for a in g:
        for e in a:
            if e == '#': ans += 1
    return ans

def get():
    grid = []
    for l in open("../input/18").read().split('\n'):
        grid.append([c for c in l])
    return grid

grid = get()
print("Part 1:",solve(grid,False))
grid = s_c(get())
print("Part 2:",solve(grid,True))
