# Advent of Code 2015, day 8
# (c) aichingert

module Day8

function part1(lines)
	ans = 0

	for i in eachindex(lines)
		j = 1
		c = 0

		while j < length(lines[i])
			if lines[i][j] == '\\'
				if '\\' == lines[i][j+1] || '"' == lines[i][j+1]
					j += 1
					c += 1
				elseif 'x' == lines[i][j+1]
					j += 3
					c += 1
				end
			elseif '"' != lines[i][j] c += 1 end

			j += 1
		end

		ans += length(lines[i]) - c
	end

	return ans
end

function part2(lines)
	ans = 0

	for i in eachindex(lines)
		c = 2
		for j in eachindex(lines[i])
			if lines[i][j] == '"' || lines[i][j] == '\\' c += 2
			else c += 1 end
		end

		ans += c - length(lines[i])
	end

	return ans
end

input = readlines("../input/08")

println("Part 1: " * string(part1(input)))
println("Part 2: " * string(part2(input)))

end
