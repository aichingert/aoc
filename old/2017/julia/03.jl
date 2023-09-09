# Advent of Code 2017, day 3
# (c) aichingert

module Day3

function ulam_spiral_distance(n::UInt32)
	k = convert(Int32, ceil(((sqrt(n)-1.0)/2.0)))
	t = 2*k+1
	m = t*t
	t -= 1

	if n >= m-t
		return (k-(m-n),-k)
	else 
		m -= t
	end

	if n >= m-t
		return (-k,-k+(m-n))
	else
		m -= t
	end

	if n >= m-t
		return (-k+(m-n),k)
	else
		return (k,k-(m-n-t))
	end
end

function part1(n::UInt32)
	x,y = ulam_spiral_distance(n)
	return abs(x) + abs(y)
end

function part2(m::UInt32)
	spiral = Dict((0,0) => 1)

	for i in UInt32(2):m
		x,y = ulam_spiral_distance(i)
		acc = 0

		for xx in -1:1
			for yy in -1:1
				if (x+xx,y+yy) in keys(spiral) 
					val = spiral[(x+xx,y+yy)]
					acc += val 
				end
			end
		end

		if acc > m return acc end
		push!(spiral, (x,y) => acc)
	end

	return 0
end

input = parse(UInt32, readline("../input/03"))

println("Part 1: $(part1(input))")
println("Part 2: $(part2(input))")

end
