#!/bin/sh

part1=$(cat 11-01.jq)
part2=$(cat 11-02.jq)

echo Part one: $(cat ../input/11 | jq -R "$part1")
echo Part two: $(cat ../input/11 | jq -R "$part2")
