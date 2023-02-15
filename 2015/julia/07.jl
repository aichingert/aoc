# Advent of Code 2015, day 7
# (c) aichingert

module Day7

function solve(instr::Dict{Any,Any}, calc::Dict{Any,Any}, cur::String)
    try
        res = parse(Int32, cur)
        return res
    catch end

    if cur in keys(calc) return calc[cur] end

    values = split(instr[cur], " ")

    # println(values)
    println(calc, cur)
    if length(values) == 1 global calc[cur] = solve(instr,calc,String(values[1])) end
    if length(values) == 2 global calc[cur] = ~solve(instr,calc,String(values[2])) 
    elseif length(values) == 3
        if values[2] == "AND" 
            global calc[cur] = solve(instr,calc,String(values[1])) & solve(instr,calc,String(values[3]))
        end
        if values[2] == "OR" global calc[cur] = solve(instr,calc,String(values[1])) | solve(instr,calc,String(values[3])) end
        if values[2] == "LSHIFT" global calc[cur] = solve(instr,calc,String(values[1])) << solve(instr,calc,String(values[3]))
        else global calc[cur] = solve(instr,calc,String(values[1])) >> solve(instr,calc,String(values[3])) end
    end

    println(calc, cur)
    return calc[cur] 
end

instr = Dict()
calc = Dict()

for l in eachline("../input/07")
    s = split(l, " -> ")
    instr[s[end]] = s[begin]
end

solve(instr, calc,"d")
part1 = solve(instr, calc,"d")
println(part1)

end
