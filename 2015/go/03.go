// Advent of Code 2015, Day 3
// (c) aichingert

package main

import (
	"fmt"
	"os"
)

type Key [2]int

func update(loc []int, c string) {
	switch c {
	case "<":
		loc[0] -= 1
	case ">":
		loc[0] += 1
	case "v":
		loc[1] -= 1
	case "^":
		loc[1] += 1
	}
}

func solve(data string, part bool) int {
	s := 0
	if part {
		s = 1
	} 

	m := map[Key]bool{}
	loc := make([][]int,s+1)
	for i := range loc {
	    loc[i] = make([]int, 2)
	}

	for i := 0 ; i < len(data) ; i++ {
		m[Key{loc[s][0],loc[s][1]}] = true
		update(loc[s], string(data[i]))
		if part {
			s = 1 - s
		}
	}

	return len(m)
}



func main() {
	dat, err := os.ReadFile("../input/03")

	if err != nil {
		panic(err)
	}

	fmt.Printf("Part 1: %d\n", solve(string(dat), false))
	fmt.Printf("Part 2: %d\n", solve(string(dat), true))
}
