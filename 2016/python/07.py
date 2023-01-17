# Advent of Code 2016, day 7
# (c) aichingert

def contains_abba(s):
    for i in range(len(s)-3):
        if s[i] == s[i+1]:
            continue
        if s[i] == s[i+3] and s[i+1] == s[i+2]:
            return True
    return False

def part1(inp):
    ans = 0
    for l in inp:
        left,rest = l.strip().split('[',1)
        v = contains_abba(left)

        for sp in rest.split('['):
            h, a = sp.split(']',1)
            if contains_abba(h):
                v = False
                break
            elif contains_abba(a):
                v = True
        if v:
            ans += 1

    return ans

def contains_ssl(addr, hypr):
    for i in range(len(addr)):
        for j in range(len(addr[i])-2):
            if addr[i][j] == addr[i][j+1]:
                continue
            if addr[i][j] == addr[i][j+2]:
                for hyp in hypr:
                    for k in range(len(hyp)-2):
                        if hyp[k] == hyp[k+2] and hyp[k] == addr[i][j+1] and hyp[k+1] == addr[i][j]:
                            return True
    return False

def part2(inp):
    ans = 0
    for l in inp:
        left,rest = l.strip().split('[',1)
        addr = [left]
        hypr = []
        for sp in rest.split('['):
            h,a = sp.split(']',1)
            addr.append(a)
            hypr.append(h)
        if contains_ssl(addr, hypr):
            ans += 1
    return ans

inp = open("../input/07").read().split('\n')
print("Part 1:", part1(inp))
print("Part 2:", part2(inp))