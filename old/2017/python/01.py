# Advent of Code 2017, day 1
# (c) aichingert

def part1(inp):
    ans = 0
    if inp[0] == inp[-1]: ans = ord(inp[0])-ord('0')

    for i in range(len(inp)-1):
        if inp[i] == inp[i+1]:
            ans += ord(inp[i]) - ord('0')
    return ans
def part2(inp):
    ans = 0
    l = len(inp)//2
    for i in range(len(inp)):
        if inp[i] == inp[(l+i)%len(inp)]:
            ans += ord(inp[i]) - ord('0')
    return ans

inp=open("../input/01").read().strip()

print("Part 1:", part1(inp))
print("Part 2:", part2(inp))
