# Advent of Code 2015, day 6
# (c) aichingert

module Day6

import Base.parse

function parse(arg::SubString)
    cords = split(arg, ",")
    return (parse(Int32, cords[1]), parse(Int32, cords[2]))
end

function part1(lines::Vector{String}, map::Matrix) 
    for line in lines
        vls = split(line, " ")
        
        if vls[1] == "toggle"
            x,y = parse(vls[2])
            cx,cy = parse(vls[4])

            for i in y:cy
                for j in x:cx
                    map[i,j] = 1 - map[i,j]
                end
            end
        else
            x,y = parse(vls[3])
            cx,cy = parse(vls[5])

             for i in y:cy
                for j in x:cx
                    if vls[2] == "on"
                        map[i,j] = 1
                    else
                        map[i,j] = 0
                    end
                end
            end
        end
    end

    return sum(map)
end

inp = readlines("../input/06")
map = zeros(Int8, 1000, 1000)
println(part1(inp,map))

end
