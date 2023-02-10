# Advent of Code 2015, day 4
# (c) aichingert

module Day4

using MD5

function solve(salt::String, zeros::String)
    ans = 1

    while bytes2hex(md5(salt*string(ans)))[1:length(zeros)] != zeros
        ans += 1
    end

    return ans
end

salt = readline("../input/04")
println("Part 1: " * string(solve(salt, "00000")))
println("Part 2: " * string(solve(salt, "000000")))

end