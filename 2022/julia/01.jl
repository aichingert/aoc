# Advent of Code 2022, day 1
# (c) aichingert

module Day1

function solve(vec,to)
	return sum([vec[i] for i in 1:to])
end

inp = []

for s in split(strip(read("../input/01", String)), "\n\n")
	push!(inp, sum(map(n->parse(UInt32,n), split(s, "\n"))))
end

sort!(inp, rev=true)

println("Part 1:" * string(solve(inp,1)))
println("Part 2:" * string(solve(inp,3)))

end
