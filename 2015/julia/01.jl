# Advent of Code 2015, day 1
# (c) aichingert

open("../input/01","r") do file
    i = 0
    d = 0
    p2 = 0

    while !eof(file)
        ch = read(file, Char)
        i += 1

        if ch == '(' d += 1 else d -= 1 end

        if d < 0 && p2 == 0 p2 = i end
    end

    println("Part 1: " * string(d))
    println("Part 2: " * string(p2))
end
