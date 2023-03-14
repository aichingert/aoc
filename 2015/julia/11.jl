# Advent of Code 2015, day 11
# (c) aichingert

module Day11

global pw = collect(readline("../input/11"))

function update()
	for i in eachindex(pw)
		if pw[length(pw)-(i-1)] == 'z' pw[length(pw)-(i-1)] = 'a'
		else pw[length(pw)-(i-1)] = Char(Int(pw[length(pw)-(i-1)]) + 1) end

		if pw[length(pw)-(i-1)] != 'a' break end
	end

	if pw[1] == 'l' || pw[1] == 'o' || pw[1] == 'i' return false end

	trip = false
	amt = 0

	for i in 1:length(pw)-2
		if Int(pw[i]) + 1 == Int(pw[i+1])  && Int(pw[i+1]) + 1 == Int(pw[i+2])
			trip = true
			break
		end
	end

	i = 1
	while i < length(pw)
		if pw[i+1] == 'l' || pw[i+1] == 'o' || pw[i+1] == 'i' return false end
		if pw[i] == pw[i+1]
			amt += 1
			i += 1
		end

		i += 1
	end
	
	trip && amt > 1
end

function solve()
	while !update() end
	String(pw)
end


println("Part 1: $(solve())")
println("Part 2: $(solve())")

end
