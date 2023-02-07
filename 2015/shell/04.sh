#!/bin/sh
# Advent of Code 2015, day 4 - aichingert

salt=$(cat "../input/04")
ans=1000

solve() {
    next="$(echo -n "$salt$ans" | md5sum)"
    g="$(echo "$next" | grep "^$1.*")"

    while test "$g" = "" ; do
        ans=$((ans + 1))
        next="$(echo -n "$salt$ans" | md5sum)"
        g="$(echo "$next" | grep "^$1.*")"
    done

    return $ans
}

solve "00000"

echo "Part 1: $ans"

solve "000000"

echo "Part 2: $ans"
