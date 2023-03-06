# Advent of Code 2017, day 1
# (c) aichingert

module Day1

function part1(s::String)
	ans = if s[begin] == s[end] Int32(s[begin]) - 48
	else 0 end

	for i in 1:length(s)-1
		if s[i] == s[i+1]
			ans += Int32(s[i]) - 48
		end
	end

	return ans
end

function part2(s::String)
	ans = 0
	half = length(s)÷2-1

	for i in eachindex(s)
		if s[i] == s[(half + i) % length(s)+1] 
			ans += Int32(s[i]) - 48
		end
	end

	return ans
end

input = readline("../input/01")

println("Part 1: $(part1(input))")
println("Part 2: $(part2(input))")

end
