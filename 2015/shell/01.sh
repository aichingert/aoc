#!/bin/sh
# Advent of Code 2015, day 1 - aichingert

inp=$(cat "../input/01")

u=$(sed 's/[^(]//g' "../input/01")
d=$(sed 's/[^)]//g' "../input/01")

echo "Part 1: $(expr ${#u} - ${#d})"
