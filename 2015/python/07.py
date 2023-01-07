# Advent of Code 2015, day 7
# (c) aichingert

def asm(instr, reg, cur):
    if cur.isdigit():
        return int(cur)

    if cur in reg:
        return reg[cur]

    ins = instr[cur].split(' ')

    if len(ins) == 1:
        reg[cur] = asm(instr,reg,ins[0])
    elif len(ins) == 2:
        reg[cur] = ~asm(instr,reg,ins[1])
    elif ins[1] == 'AND':
        reg[cur] = asm(instr,reg,ins[0]) & asm(instr,reg,ins[2])
    elif ins[1] == 'OR':
        reg[cur] = asm(instr,reg,ins[0]) | asm(instr,reg,ins[2])
    elif ins[1] == 'LSHIFT':
        reg[cur] = asm(instr,reg,ins[0]) << asm(instr,reg,ins[2])
    else:
        reg[cur] = asm(instr,reg,ins[0]) >> asm(instr,reg,ins[2])
    return reg[cur]

instr = {}
reg = {}

for line in open("../input/07").read().split('\n'):
    rhs,lhs = line.split(' -> ', 1)
    instr[lhs] = rhs

a = asm(instr, reg, 'a')
print("Part 1:",a)
reg = {'b': a}
print("Part 1:",asm(instr,reg,'a'))
