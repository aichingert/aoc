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

	l=$(expr $(expr "2" \* "$n1") \* "$n2")
	b=$(expr $(expr "2" \* "$n2") \* "$n3")
	w=$(expr $(expr "2" \* "$n1") \* "$n3")
	e=$(expr "$n1" \* "$n2")

	f=$(expr $(expr "$n1" + "$n1") + $(expr "$n2" + "$n2"))
	ri=$(expr $(expr "$n1" \* "$n2") \* "$n3")
	
	p1=$(expr "$p1" + $(expr "$e" + $(expr $(expr "$l" + "$b") + "$w")))
	p2=$(expr "$p2" + $(expr "$f" + "$ri"))
done

echo "Part 1: $p1"
echo "Part 2: $p2"
