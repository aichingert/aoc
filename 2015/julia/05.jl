# Advent of Code 2015, day 5
# (c) aichingert

module Day5

function part1(lines::Vector{String})
    ans = 0

    for l in lines
        vc = 0
        dd = false

        if contains(l, r"ab|cd|pq|xy") continue end

        for i in 1:length(l)
            if contains(string(l[i]), r"a|e|i|o|u") vc += 1 end
            if i+1 < length(l) && l[i] == l[i+1]
                dd = true
            end
        end

        if dd && vc > 2 ans += 1 end
    end
    
    return ans
end

function part2(lines::Vector{String})
    ans = 0

    for l in lines

    end
end

lines = readlines("../input/05")
println("Part 1: " * string(part1(lines)))

end
