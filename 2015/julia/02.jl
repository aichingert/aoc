# Advent of Code 2015, day 2
# (c) aichingert

open("../input/02", "r") do f
    p1 = 0
    p2 = 0

    for l in eachline(f)
        n = sort([parse(Int64, s) for s in split(l, "x")])
        p1 += 2*n[1]*n[2] + 2*n[2]*n[3] + 2*n[1]*n[3] + n[1]*n[2]
        p2 += 2*n[1]+2*n[2] + n[1]*n[2]*n[3]
    end

    println("Part 1: " * string(p1))
    println("Part 2: " * string(p2))
end
