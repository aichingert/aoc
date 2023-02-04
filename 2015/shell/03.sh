#!/bin/sh
# Advent of Code 2015, day 3 - aichingert

inp=$(cat "../input/03")
touch p1
touch p2

x=1000
y=1000
px=1000
py=1000
xx=1000
yy=1000
who=0

echo "$x$y" >> p1
echo "$px$py" >> p2

for ch in $(echo "$inp" | fold -w1); do
    case "$ch" in
        ">") x=$(($x + 1)) 
            if test "$who" -eq 1 ; then
                px=$(($px + 1)) 
            else 
                xx=$(($xx + 1))
            fi
        ;;
        "<") x=$(($x - 1)) 
            if test "$who" -eq 1 ; then 
                px=$(($px - 1)) 
            else 
                xx=$(($xx - 1))
            fi 
        ;;
        "^") y=$(($y + 1)) 
            if test "$who" -eq 1 ; then 
                py=$(($py + 1)) 
            else 
                yy=$(($yy + 1))
            fi 
        ;;
        "v") y=$(($y - 1))
            if test "$who" -eq 1 ; then
                py=$(($py - 1)) 
            else 
                yy=$(($yy - 1))
            fi
        ;;
    esac 

    echo "$x$y" >> p1
    echo "$px$py" >> p2
    echo "$xx$yy" >> p2

    who=$((1 - $who))
done

echo "Part 1: $(sort -nu p1 | wc -l)"
echo "Part 2: $(sort -nu p2 | wc -l)"

rm p1 
rm p2
