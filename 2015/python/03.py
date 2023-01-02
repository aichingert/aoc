# Advent of Code 2015, day 3
# (c) aichingert

def update(loc, c):
    match c:
        case '<':
            loc[0]-=1
        case '>':
            loc[0]+=1
        case 'v':
            loc[1]-=1
        case '^':
            loc[1]+=1

def solve(inp, part):
    if part: s=1 
    else: s=0
    loc = [[0,0] for _ in range(-1,s)]
    p = [(loc[s][0],loc[s][1])]
    
    for i in range(len(inp)):
        update(loc[s], inp[i])
        p.append((loc[s][0],loc[s][1]))
        if part: s = 1 - s
    return len(set(p))

inp = open("../input/03").read()
print("Part 1:",solve(inp,False))
print("Part 2:",solve(inp,True))
