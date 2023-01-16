# Advent of Code 2016, day 6
# (c) aichingert

def solve(inp,part):
    ans = ""
    count = [{} for _ in range(len(inp[0]))]
    for i in range(len(inp)):
        for j in range(len(inp[i])):
            if inp[i][j] in count[j]: count[j][inp[i][j]] += 1
            else: count[j][inp[i][j]] = 1

    for i in range(len(count)):
        if part: loc = ("", 0)
        else: loc = ("", 10000)

        for k in count[i].keys():
            if part and loc[1] < count[i][k]: loc = (k, count[i][k])
            if not part and loc[1] > count[i][k]: loc = (k, count[i][k])
        ans += loc[0]
    return ans


inp = open("../input/06").read().split('\n')
print("Part 1:", solve(inp,True))
print("Part 2:", solve(inp,False))