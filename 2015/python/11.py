# Advent of Code 2015, day 11
# (c) aichingert

def inc(s):
    n = ""
    a = 1
    for i in range(len(s)-1,-1,-1):
        if s[i] == 'z' and a == 1:
            n += 'a'
        else:
            n += chr(ord(s[i])+a)
            a = 0
    return n[::-1]

def valid(s):
    valid = False

    if "i" in s or "o" in s or "l" in s:
        return False

    for i in range(len(s)-2):
        if ord(s[i]) + 1 == ord(s[i+1]) and ord(s[i]) + 2 == ord(s[i+2]):
            valid = True
            break

    if not valid: return False

    i = 0
    d = 0
    while i < len(s)-1:
        if s[i] == s[i+1]:
            d+=1
            i+=1
        i += 1

    return d > 1

def solve(inp):
    while not valid(inp):
        inp = inc(inp)
        
    return inp

inp = open("../input/11").read()

n = solve(inp)
print("Part 1:",n)
n = inc(n)
print("Part 2:",solve(n))
