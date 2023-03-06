# Advent of Code 2017, day 2
# (c) aichingert

module Day2

partone = 0
parttwo = 0

function div(cur::Vector{Int})
	for i in eachindex(cur)
		for j in eachindex(cur)
			if i == j continue end

			if cur[i] % cur[j] == 0 return cur[i] ÷ cur[j] end
			if cur[j] % cur[i] == 0 return cur[j] ÷ cur[i] end
		end
	end

	return 0
end

for l in readlines("../input/02")
	cur = parse.(Int, split(l,'\t'))
	global partone += maximum(cur) - minimum(cur)
	global parttwo += div(cur)
end

println("Part 1: $(partone)")
println("Part 2: $(parttwo)")

end
