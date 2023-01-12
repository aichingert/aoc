# Advent of Code 2015, day 19
# (c) aichingert 

def rep(text,s,r,c):
    co = 0
    ne = ""
    j = 0
    for n in range(len(text)):
        if j > 0:
            j -= 1
            continue
        if text[n:n+len(s)] == s:
            co += 1
            if co == c:
                ne += r
                j = len(s)-1
            else:
                ne += text[n]
        else:
            ne += text[n]
    return ne

def part1(text,rules):
    p = []

    for key in rules.keys():
        for rule in rules[key]:
            t = 1
            for n in range(len(text)):
                if text[n:n+len(key)] == key:
                    p.append(rep(text,key,rule,t))
                    t+=1
    return len([*set(p)])
 

lines,text = open("../input/19").read().split('\n\n', 1)
rules = {}

for line in lines.split('\n'):
    val = line.split(' => ')

    if val[0] not in rules:
        rules[val[0]] = [val[1]]
    else:
        rules[val[0]].append(val[1])

print("Part 1: ", part1(text,rules))
print("Part 2: ", "NOT SOLVED")
