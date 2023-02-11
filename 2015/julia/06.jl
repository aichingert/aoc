# Advent of Code 2015, day 6
# (c) aichingert

module Day6

import Base.parse

function parse(arg::SubString)
    cords = split(arg, ",")
    return (parse(Int32, cords[1]), parse(Int32, cords[2]))
end

function solve(lines::Vector{String}, map::Matrix, part::Bool) 
    for line in lines
        vls = split(line, " ")
        if length(vls) == 4 insert!(vls, 1, "") end
        x,y = parse(vls[3])
        cx,cy = parse(vls[5])

        for i in y:cy
            for j in x:cx
                if vls[2] == "on"
                    if part map[i,j] = 1 else map[i,j] += 1 end
                elseif vls[2] == "off"
                    if part map[i,j] = 0 else map[i,j] = max(0, map[i,j] - 1)  end
                else
                    if part map[i,j] = 1 - map[i,j] else map[i,j] += 2 end
                end
            end
        end
    end

    return sum(map)
end

map = zeros(Int8,1000,1000)
inp = readlines("../input/06")

println("Part 1: " * string(solve(inp,copy(map), true)))
println("Part 2: " * string(solve(inp,map, false)))

end
