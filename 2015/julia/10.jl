# Advent of Code 2015, day 10
# (c) aichingert

module Day10

global sequence = [parse(Int, s) for s in readline("../input/10")]

function lookandsay(n)
	for _ in 1:n
		ne = []
		i = 1

		while i <= length(sequence)
			amt = 1
			for j in i+1:length(sequence)
				if sequence[j] != sequence[i] break end
				amt += 1
				i += 1
			end

			i += 1
			push!(ne, amt)
			push!(ne, sequence[i-1])
		end
		global sequence = ne
	end

	length(sequence)
end

println("Part 1: $(lookandsay(40))")
println("Part 2: $(lookandsay(10))")

end
