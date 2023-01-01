def solve(inp, part):
    loc = 0

    for i,c in enumerate(inp):
        match c:
            case '(':
                loc += 1
            case ')':
                loc -= 1

        if part and loc < 0:
            return i + 1
    return loc

inp = open("../input/01").read()

print("Part 1:",solve(inp, False))
print("Part 2:",solve(inp, True))
