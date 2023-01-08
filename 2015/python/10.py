# Advent of Code 2015, day 10
# (c) aichingert

def solve(inp, n):
    ans = inp
    for _ in range(n):
        i = 0
        ne = ""
        while i < len(ans):
            amt = 1
            of = ans[i]
            for j in range(i+1, len(ans)):
                if ans[j] != of: break
                amt += 1
                i+=1
            i+=1
            ne += str(amt) + of
        ans = ne

    return len(ans)

inp = open("../input/10").read()
print("Part 1:", solve(inp,40))
print("Part 2:", solve(inp,50))
