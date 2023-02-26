# Advent of Code 2022, day 3
# (c) aichingert

module Day3

function getvalue(s,fst,scn)
	ch = '_'

	for c in s
		if c in fst && c in scn ch = c end
	end

	if isuppercase(ch) return 27 + Int(ch) - Int('A') 
	else return 1 + Int(ch) - Int('a') end
end

function part1(inp::Vector{String})
	ans = 0

	for s in inp
		half = length(s)÷2
		ans += getvalue(s,s[begin:half],s[half+1:end])
	end

	return ans
end

function part2(inp::Vector{String})
	ans = 0

	for i in 1:3:length(inp)
		ans += getvalue(inp[i],inp[i+1],inp[i+2])
	end

	return ans
end

inp = readlines("../input/03")

println("Part 1: $(part1(inp))")
println("Part 2: $(part2(inp))")

end
