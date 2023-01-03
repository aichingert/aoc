# Advent of Code 2015, day 5
# (c) aichingert

I = ["ab","cd","pq","xy"]

def part1(lines):
    c = 0
    for s in lines:
        if s[-1] == 'a' or s[-1] == 'e' or s[-1] == 'i' or s[-1] == 'o' or s[-1] == 'u': vc = 1
        else: vc = 0
        v = True
        dd = False 

        for i in range(len(s)-1):
            if s[i] == 'a' or s[i] == 'e' or s[i] == 'i' or s[i] == 'o' or s[i] == 'u':
                vc += 1
            if s[i:i+2] in I:
                v = False
                break
            if s[i] == s[i+1]:
                dd = True

        if v and dd and vc >= 3:
            c += 1
    return c

def part2(lines):
    c = 0
    for s in lines:
        f = False
        t = False

        for i in range(len(s)-2):
            for j in range(i+2,len(s)-1):
                if (s[i] == s[j] and s[i+1] == s[j+1]):
                    f = True
            if s[i] == s[i+2]:
                t = True

        if f and t:
            c += 1
    return c

inp = open("../input/05").read().split('\n')

print("Part 1:",part1(inp))
print("Part 2:",part2(inp))
