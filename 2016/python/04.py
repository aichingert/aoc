# Advent of Code 2016, day 4
# (c) aichingert


def part1(inp):
    ans = 0
    for l in inp:
        cur = {}
        real = True 
        vls = l.split('-')
        lhs,rhs = vls.pop(-1).split('[')
        for v in ''.join(vls):
            if v in cur:
                cur[v] += 1
            else:
                cur[v] = 1
        while len(rhs) > 1:
            for k in cur.keys():
                if not (rhs[0] in cur and cur[rhs[0]] >= cur[k]):
                    real = False
                    break
            if rhs[0] in cur:
                cur.pop(rhs[0])
            rhs = rhs[1:]
        if real: ans += int(lhs)
    return ans

def decrypt(ch, n):
    for _ in range(n):
        if ord(ch) == ord('z'):
            ch = 'a'
        else:
            ch = chr(ord(ch)+1)
    return ch

def part2(inp):
    for l in inp:
        cur = ""
        lhs,_ = l.split('[',1)
        vls = lhs.split('-')
        ans = int(vls.pop(-1))

        for ch in ''.join(vls):
            cur += decrypt(ch, ans)
        if "northpoleobjectstorage" == cur:
            return ans
    return None

inp = open("../input/04").read().split('\n')
print("Part 1:", part1(inp))
print("Part 2:", part2(inp))
