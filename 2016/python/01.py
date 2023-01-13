# Advent of Code 2016, day 1
# (c) aichingert

def part1(inp):
    x,y = 0,0
    d = 0
    for i in range(len(inp)):
        if inp[i][:1] == 'R': d = (d + 90) % 360
        else: d = ((d - 90) + 360) % 360

        if d == 0: x += int(inp[i][1:])
        elif d == 180: x -= int(inp[i][1:])
        elif d== 90: y += int(inp[i][1:])
        else: y -= int(inp[i][1:])
    return abs(x) + abs(y)

def part2(inp):
    x,y = 0,0
    d = 0
    i = 0
    loc = set()
    while True:
        if (x,y) in loc:
            return abs(x) + abs(y)

        if inp[i][:1] == 'R': d = (d + 90) % 360
        else: d = ((d - 90) + 360) % 360

        if d == 0: 
            for xx in range(0,int(inp[i][1:])):
                if (x-xx,y) in loc: return abs(x-xx) + abs(y)
                loc.add((x-xx,y))
            x -= int(inp[i][1:])
        elif d == 180: 
            for xx in range(0,int(inp[i][1:])):
                if (x+xx,y) in loc: return abs(x+xx) + abs(y)
                loc.add((x+xx,y))
            x += int(inp[i][1:])
        elif d== 90: 
            for yy in range(0,int(inp[i][1:])):
                if (x,y-yy) in loc: return abs(x) + abs(y-yy)
                loc.add((x,y-yy))
            y -= int(inp[i][1:])
        else: 
            for yy in range(0,int(inp[i][1:])):
                if (x,y+yy) in loc: return abs(x) + abs(y+yy)
                loc.add((x,y+yy))
            y += int(inp[i][1:])
        i += 1
        if i == len(inp): i = 0
    return abs(x) + abs(y)

inp = open("../input/01").read().split(', ')
print("Part 1:", part1(inp))
print("Part 2:", part2(inp))
