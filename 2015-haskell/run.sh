#!/bin/sh
# (c) aichingert

file=$(basename $1 .hs)

ghc -dynamic $1 --make Aoc.hs

./src/$file

rm "./src/$file.hi"
rm "./src/$file.o"
rm "./src/$file"

rm "Aoc.hi"
rm "Aoc.o"
