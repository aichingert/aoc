# Advent of Code 2022, day 2
# (c) aichingert

module Day2

function part1(inp)
	return sum(map(n-> n[2] + (3 - (2 + n[1] - n[2]) % 3) % 3 * 3, inp))
end

function part2()
	return sum(map(n-> (n[1]+n[2]) % 3 + 1 + (n[2] - 1) % 3 * 3, inp))
end

function map_value(s::SubString) 
	if s == "A" || s == "X" return 1 
	elseif s == "B" || s == "Y" return 2 
	else return 3 end
end

inp = []

for l in eachline("../input/02")
	(lhs,rhs) = split(l, " ")
	push!(inp, (map_value(lhs), map_value(rhs)))
end

println("Part 1:" * string(part1(inp)))
println("Part 2:" * string(part2()))

end
