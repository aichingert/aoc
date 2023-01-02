// Advent of Code 2015, Day 1
// (c) aichingert

package main

import (
	"fmt"
	"os"
)

func solve(dat string, part bool) int {
	loc := 0

	for i,c := range dat {
		switch c {
		case '(':
			loc += 1
		case ')':
			loc -= 1
		}

		if part && loc < 0 {
			return i+1
		}
	}

	return loc
}

func main() {
	dat, err := os.ReadFile("../input/01")

	if err != nil {
		panic(err)
	}

	fmt.Printf("Part 1: %d\n", solve(string(dat), false))
	fmt.Printf("Part 2: %d\n", solve(string(dat), true))
}
