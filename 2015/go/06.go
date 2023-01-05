// Advent of Code 2015, day 6
// (c) aichingert

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

func solve(instr []string, part bool) int {
	c := 0
	m := make([][]int,1000)
	for i := range m {
		m[i] = make([]int,1000)
	}

	for _,s := range instr {
		ch,fx,fy,tx,ty := parse(s)

		for x := fx ; x <= tx ; x++ {
			for y := fy ; y <= ty ; y++ {
				if ch == "on" { 
					if part { m[y][x] = 1 } else { m[y][x] += 1 }
				} else if ch == "off" { 
					if part { m[y][x] = 0 } else { m[y][x] =  max(0, m[y][x]-1) }
				} else { 
					if part { m[y][x] = 1 - m[y][x] } else { m[y][x] += 2 }
				}
			}
		}
	}

	for i := range m {
		for j := range m[i] {
			c += m[i][j]
		}
	}

	return c
}

func parse(line string) (string, int, int, int, int) {
	vls := strings.Split(line, " ")
	off := 0
	if vls[0] == "toggle" { off += 1 } 
	f := strings.Split(vls[2-off], ",")
	s := strings.Split(vls[4-off], ",")

	a,_ := strconv.Atoi(strings.TrimSpace(f[0]))
	b,_ := strconv.Atoi(strings.TrimSpace(f[1]))
	c,_ := strconv.Atoi(strings.TrimSpace(s[0]))
	d,_ := strconv.Atoi(strings.TrimSpace(s[1]))
	return vls[1], a, b, c, d
}

func max(a int, b int) int {
	if a > b {
		return a
	}

	return b
}

func main() {
	dat, err := os.ReadFile("../input/06")

	if err != nil {
		panic(err)
	}

	instr := strings.Split(string(dat), "\n")
	fmt.Printf("Part 1: %d\n", solve(instr, true))
	fmt.Printf("Part 2: %d\n", solve(instr, false))
}