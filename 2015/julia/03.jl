# Advent of Code 2015, day 3
# (c) aichingert

open("../input/03", "r") do f
	loc = [[0,0],[0,0]]
	pos = [0,0]
	cur = 0
    p1 = Dict([0,0] => true)
    p2 = Dict([0,0] => true)

	while !eof(f)
		ch = read(f, Char)
		
		if ch == '^' 
			loc[cur+1][1] += 1
			pos[1] += 1
		elseif ch == 'v'
			loc[cur+1][1] -= 1
			pos[1] -= 1
		elseif ch == '>'
			loc[cur+1][2] += 1
			pos[2] += 1
		else 
			loc[cur+1][2] -= 1
			pos[2] -= 1
		end
		
		push!(p1, [pos[1],pos[2]] => true)
		push!(p2, [loc[cur+1][1],loc[cur+1][2]] => true)
		cur = 1 - cur
    end

    println("Part 1: " * string(length(p1)))
    println("Part 2: " * string(length(p2)))
end
