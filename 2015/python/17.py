# Advent of Code 2015, day 17
# (c) aichingert

import itertools

F = 150 

def solve(con,part):
    ans = 0
    for i in range(len(con)):
        if part and ans != 0:
            return ans

        for comb in itertools.combinations(con, i):
            if F == sum(comb):
                ans += 1
    return ans

con = [int(c) for c in open("../input/17").read().strip().split('\n')]

print("Part 1:",solve(con, False))
print("Part 2:",solve(con, True))
