# Advent of Code 2015, day 7
# (c) aichingert

module Day7

function solve(instr::Dict{Any,Any}, cur::String)
    if tryparse(UInt16, cur) !== nothing
	return parse(UInt16, cur)
    end

    if cur in keys(calc) return calc[cur] end

    values = split(instr[cur], " ")

    if length(values) == 1 global calc[cur] = solve(instr,String(values[1]))
    elseif length(values) == 2 global calc[cur] = ~solve(instr,String(values[2])) 
    else
        if values[2] == "AND" global calc[cur] = solve(instr,String(values[1])) & solve(instr,String(values[3]))
	elseif values[2] == "OR" global calc[cur] = solve(instr,String(values[1])) | solve(instr,String(values[3]))
	elseif values[2] == "LSHIFT" global calc[cur] = solve(instr,String(values[1])) << solve(instr,String(values[3]))
    	else global calc[cur] = solve(instr,String(values[1])) >> solve(instr,String(values[3])) end
    end

    return calc[cur] 
end

instr = Dict()
calc = Dict()

for l in eachline("../input/07")
    s = split(l, " -> ")
    instr[s[end]] = s[begin]
end

part1 = solve(instr, "a")
empty!(calc)
calc["b"] = part1

println("Part 1: " * string(part1))
println("Part 2: " * string(solve(instr, "a")))

end
