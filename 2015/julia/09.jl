# Advent of Code 2015, day 9
# (c) aichingert

module Day9

using Combinatorics

function solve(dist, cities) 
	partone = typemax(Int32)
	parttwo = 0

	for perm in collect(permutations(cities))
		cur = 0
		for i in 1:length(perm)-1 cur += dist[(perm[i],perm[i+1])] end
		partone = min(partone, cur) 
		parttwo = max(parttwo, cur)
	end

	(partone,parttwo)
end

dist = Dict()
cities = []

for l in readlines("../input/09")
	locs,distance = split(l, " = ")
	from,to = split(locs, " to ")
	push!(dist, (from,to) => parse(Int32, distance))
	push!(dist, (to,from) => parse(Int32, distance))
	if !(from in cities) push!(cities, from) end
	if !(to in cities) push!(cities, to) end
end

partone,parttwo = solve(dist, cities)

println("Part 1: $(partone)")
println("Part 2: $(parttwo)")

end
