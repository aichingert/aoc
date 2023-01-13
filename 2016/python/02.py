# Advent of Code 2016, day 2
# (c) aichingert

F = [[1,2,3],[4,5,6],[7,8,9]]
S = [['0','0','1','0','0'],['0','2','3','4','0'],['5','6','7','8','9'],['0','A','B','C','0'],['0','0','D','0','0']]

def part1(inp):
    ans = ""
    loc = [1,1]

    for i in range(len(inp)):
        for j in range(len(inp[i])):
            match inp[i][j]:
                case 'U':loc[1] = max(0, loc[1]-1)
                case 'L':loc[0] = max(0, loc[0]-1)
                case 'R':loc[0] = min(2, loc[0]+1)
                case 'D':loc[1] = min(2, loc[1]+1)
        ans += str(F[loc[1]][loc[0]])
    return ans

def part2(inp):
    ans = ""
    loc = [0,2]

    for i in range(len(inp)):
        for j in range(len(inp[i])):
            match inp[i][j]:
                case 'U':
                    if loc[1]>0 and S[loc[1]-1][loc[0]] != '0': loc[1] -= 1
                case 'L':
                    if loc[0]>0 and S[loc[1]][loc[0]-1] != '0': loc[0] -= 1
                case 'R':
                    if loc[0]+1<len(S) and S[loc[1]][loc[0]+1] != '0': loc[0] += 1
                case 'D':
                    if loc[1]+1<len(S) and S[loc[1]+1][loc[0]] != '0': loc[1] += 1
        ans += str(S[loc[1]][loc[0]])
    return ans

inp = open("../input/02").read().split('\n')
print("Part 1:",part1(inp))
print("Part 2:",part2(inp))
