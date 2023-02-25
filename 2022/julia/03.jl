# Advent of Code 2022, day 3
# (c) aichingert

module Day3

function getchar(s,fst,scn)
	ch = '_'

	for c in s
		if c in fst && c in scn ch = c end
	end

	return ch
end

function part1(inp::Vector{String})
	ans = 0

	for s in inp
		half = length(s)÷2
		ch = getchar(s,s[begin:half],s[half+1:end])

		if isuppercase(ch) ans += 27 + Int(ch) - Int('A') 
		else ans += 1 + Int(ch) - Int('a') end
	end

	return ans
end

function part2(inp::Vector{String})
	ans = 0

	for i in 1:3:length(inp)
		ch = getchar(inp[i],inp[i+1],inp[i+2])
		
		if isuppercase(ch) ans += 27 + Int(ch) - Int('A') 
		else ans += 1 + Int(ch) - Int('a') end
	end

	ans
end

inp = readlines("../input/03")

println("Part 1: $(part1(inp))")
println("Part 2: $(part2(inp))")

end
