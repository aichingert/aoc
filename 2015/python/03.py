# Advent of Code 2015, day 3
# (c) aichingert

def part1(inp):
    x,y = 0,0
    p = [(x,y)]
    for i in range(len(inp)):
        match inp[i]:
            case '<':
                x-=1
            case '>':
                x+=1
            case 'v':
                y-=1
            case '^':
                y+=1
        p.append((x,y))
    return len(set(p))

def part2(inp):
    loc = [[0,0],[0,0]]
    s = 0
    p = [(0,0)]

    for i in range(len(inp)):
        match inp[i]:
            case '<':
                loc[s][0]-=1
            case '>':
                loc[s][0]+=1
            case 'v':
                loc[s][1]-=1
            case '^':
                loc[s][1]+=1
        p.append((loc[s][0],loc[s][1]))
        s = 1 - s
    return len(set(p))



inp = open("../input/03").read()
print("Part 1:",part1(inp))
print("Part 2:",part2(inp))
