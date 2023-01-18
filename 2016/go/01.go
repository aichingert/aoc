// Advent of Code 2016, day 1
// (c) aichingert

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

type Key [2]int

func part1(inp []string) int {
	x := 0
	y := 0
	d := 0

	for i := range inp {
		if inp[i][:1] == "R" { 
			d = (d + 90) % 360 
		} else { 
			d = ((d - 90) + 360) % 360 
		}

		n,_ := strconv.Atoi(inp[i][1:])
		if d == 0 { 
			x += n 
		} else if d == 180 { 
			x -= n 
		} else if d == 270 {
			y -= n 
		} else {
			y += n 
		}
	}

	return abs(x) + abs(y)
}

func abs(n int) int {
	if n < 0 { return n * (-1) }
	return n
}

func part2(inp []string) int {
	x := 0
	y := 0
	d := 0
	i := 0
	loc := map[Key]bool{}

	for true {
		if inp[i][:1] == "R" { 
			d = (d + 90) % 360 
		} else { 
			d = ((d - 90) + 360) % 360 
		}

		n,_ := strconv.Atoi(inp[i][1:])

		switch d {
		case 0:
			for xx := 0 ; xx < n ; xx++ {
				_, ok := loc[Key{x-xx,y}]
				if ok {
					return abs(x-xx) + abs(y)
				} else {
					loc[Key{x-xx,y}] = true
				}
			}
			x -= n
		case 180:
			for xx := 0 ; xx < n ; xx++ {
				_, ok := loc[Key{x+xx,y}]
				if ok {
					return abs(x+xx) + abs(y)
				} else {
					loc[Key{x+xx,y}] = true
				}
			}
			x += n
		case 90:
			for yy := 0 ; yy < n ; yy++ {
				_, ok := loc[Key{x,y-yy}]
				if ok {
					return abs(x) + abs(y-yy)
				} else {
					loc[Key{x,y-yy}] = true
				}
			}
			y -= n
		case 270:
			for yy := 0 ; yy < n ; yy++ {
				_, ok := loc[Key{x,y+yy}]
				if ok {
					return abs(x) + abs(y+yy)
				} else {
					loc[Key{x,y+yy}] = true
				}
			}
			y += n
		}

		i += 1
		if i == len(inp) { i = 0 }
	}

	return 0
}
func main() {
	dat, err := os.ReadFile("../input/01")

	if err != nil {
		panic(err)
	}

	inp := strings.Split(strings.Trim(string(dat), "\r\n"), ", ")

	fmt.Printf("Part 1: %d\n", part1(inp))
	fmt.Printf("Part 2: %d\n", part2(inp))
}
