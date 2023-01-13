# Advent of Code 2016, day 3
# (c) aichingert

def is_triangle(a,b,c):
    return a+b>c and a+c>b and b+c>a

def part1(inp):
    ans = 0
    for l in inp:
        vls = list(map(int, list(filter(lambda x: x != '', l.split(' ')))))
        if is_triangle(vls[0], vls[1], vls[2]):
            ans += 1
    return ans

def part2(inp):
    ans = 0
    for i in range(0,len(inp), 3):
        f = list(map(int, list(filter(lambda x: x != '', inp[i].split(' ')))))
        s = list(map(int, list(filter(lambda x: x != '', inp[i+1].split(' ')))))
        t = list(map(int, list(filter(lambda x: x != '', inp[i+2].split(' ')))))
        if is_triangle(f[0],s[0],t[0]): ans += 1
        if is_triangle(f[1],s[1],t[1]): ans += 1
        if is_triangle(f[2],s[2],t[2]): ans += 1
    return ans

inp = open("../input/03").read().split('\n')
print("Part 1:",part1(inp))
print("Part 2:",part2(inp))
