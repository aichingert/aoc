#!/bin/sh
# Advent of Code 2015, day 2 - aichingert

exec <"../input/02"
p1="0"
p2="0"

while read a b ; do
	n1=""
	n2=""
	n3=""

	for n in $(echo "$a" | tr "x" "\n" | sort -n) ; do
		if test "$n1" = "" ; then
			n1="$n"
		elif test "$n2" = "" ; then
			n2="$n"
		else
			n3="$n"
		fi
	done
	
	p1=$(($p1 + 2 * $n1 * $n2 + 2 * $n2 * $n3 + 2 * $n1 * $n3 + $n1 * $n2))
	p2=$(($p2 + $n1 + $n1 + $n2 + $n2 + $n1 * $n2 * $n3))
done

echo "Part 1: $p1"
echo "Part 2: $p2"
