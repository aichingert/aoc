# Advent of Code 2015, day 4
# (c) aichingert

import hashlib

def get_hex(s,index):
    return hashlib.md5((s + str(index)).encode('utf-8')).hexdigest()

def solve(key,zeros):
    n = 0
    done = False
    while not done:
        done = True
        n+=1
        s = set(get_hex(key,n)[:zeros]) 
        for i in s:
            if i != "0":
                done = False
                break
    return n

key = open("../input/04").read().strip()
print("Part 1:",solve(key,5))
print("Part 2:",solve(key,6))
