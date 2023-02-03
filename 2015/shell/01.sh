#!/bin/sh
# Advent of Code 2015, day 1 - aichingert

inp=$(cat "../input/01")
loc=0
f=0

u=$(sed 's/[^(]//g' "../input/01")
d=$(sed 's/[^)]//g' "../input/01")

p1=$(expr ${#u} - ${#d})

for ch in $(echo "$inp" | fold -w1); do
	loc=$(expr $loc + 1)

	if test $ch = "(" ; then 
		f=$(expr $f + "1")
	else
		f=$(expr $f - "1")
	fi

	if test $f -lt 0 ; then
		p2=$loc
		break
	fi
done

echo "Part 1: $p1"
echo "Part 2: $p2"
