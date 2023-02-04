#!/bin/sh
# Advent of Code 2015, day 3 - aichingert

inp=$(cat "../input/03")
touch hashmap

x=1000
y=1000

for ch in $(echo "$inp" | fold -w1); do
    echo "$x$y" >> hashmap
    case $ch in
        ">") x=$(($x + 1)) ;;
        "<") x=$(($x - 1)) ;;
        "^") y=$(($y + 1)) ;;
        "v") y=$(($y - 1)) ;;
    esac 
done

echo "Part 1: $(sort -nu hashmap | wc -l)"

rm hashmap
