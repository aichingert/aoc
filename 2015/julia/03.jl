# Advent of Code 2015, day 3
# (c) aichingert

open("../input/03", "r") do f
	loc = [[0,0],[0,0]]
    p1 = Dict([0,0] => true)
    p2 = 0

	while !eof(f)
		ch = read(f, Char)
		
		if ch == '^' 
			loc[1][1] += 1
		elseif ch == 'v'
			loc[1][1] -= 1
		elseif ch == '>'
			loc[1][2] += 1
		else 
			loc[1][2] -= 1
		end
		
		push!(p1, [loc[1][1],loc[1][2]] => true)
    end

    println("Part 1: " * string(length(p1)))
    println("Part 2: " * string(p2))
end
