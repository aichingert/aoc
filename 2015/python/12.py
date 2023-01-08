# Advent of Code 2015, day 12
# (c) aichingert

import json

def part1(data, valid):
    ans = ""
    for ch in data:
        if ch in valid:
            ans += ch
        else:
            ans += ' '
    return sum([int(i) for i in ans.split()])

def rem_red(js):
    if isinstance(js,dict):
        for k,v in js.items():
            if k == "red" or v == "red":
                return None
        ans = dict()
        for k, v in js.items():
            ans[k] = rem_red(v)
        return ans
    elif isinstance(js,list):
        ans = []
        for v in js:
            ans.append(rem_red(v))
        return ans
    else:
        return js

def part2(data, valid):
    return part1(str(rem_red(json.loads(data.strip()))), valid)


inp = open("../input/12").read()
valid = "-0123456789"

print("Part 1:",part1(inp, valid))
print("Part 2:",part2(inp, valid))
