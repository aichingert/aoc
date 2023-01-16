# Advent of Code 2016, day 5
# (c) aichingert

import hashlib

def get_hex(s,index):
    return hashlib.md5((s + str(index)).encode('utf-8')).hexdigest()

def part1(inp):
    n = 0
    ans = ""
    while True:
        n += 1
        s = get_hex(inp,n)
        con = False
        if s[:5] != "00000":
            continue
        ans += s[5]
        if len(ans) == 8:
            return ans
def part2(inp):
    n = 0
    ans = [""]*8
    while True:
        n += 1
        s = get_hex(inp,n)

        if s[:5] != "00000" or not s[5].isdigit():
            continue

        loc = int(s[5])

        if loc > 7 or loc < 0 or ans[loc] != "":
            continue
        ans[loc] = s[6]

        if "" not in ans:
            return "".join(ans)


inp = open("../input/05").read()
print("Part 1:", part1(inp))
print("Part 2:", part2(inp))
