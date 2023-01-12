# Advent of Code 2015, day 8
# (c) aichingert

def part1(inp):
    c = 0
    ch = 0
    for l in inp:
        c += len(l)
        i = 0
        while i < len(l)-1:
            if l[i] == "\\":
                if l[i+1] == "\\" or l[i+1] == "\"":
                    i += 1
                    ch += 1
                elif l[i+1] == "x":
                    i += 3
                    ch += 1
            elif l[i] != "\"":
                ch += 1
            i+=1
    return c - ch

def part2(inp):
    c = 0
    n = 0
    for l in inp:
        c += len(l)
        for i in range(len(l)):
            if l[i] == "\"" or l[i] == "\\":
                n += 2
            else:
                n += 1
        n += 2
    return n - c

inp = open('../input/08').read().split('\n')
print("Part 1:",part1(inp))
print("Part 2:",part2(inp))
