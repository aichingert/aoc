// Advent of Code 2015, Day 2
// (c) aichingert

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
	"sort"
)


func main() {
	dat, err := os.ReadFile("../input/02")

	if err != nil {
		panic(err)
	}

	vls := strings.Split(string(dat), "\n")
	paper := 0
	ribbon := 0

	for i:=0; i < len(vls)-1; i++ {
		s := strings.Split(vls[i], "x")
		l,_ := strconv.Atoi(strings.TrimSpace(s[0]))
		w,_ := strconv.Atoi(strings.TrimSpace(s[1]))
		h,_ := strconv.Atoi(strings.TrimSpace(s[2]))
		n := []int{l,w,h}
		sort.Ints(n)

		paper += 2*n[0]*n[1] + 2*n[1]*n[2] + 2*n[2]*n[0] + n[0]*n[1]
		ribbon += 2*n[0]+2*n[1] + n[0]*n[1]*n[2]
	}

	fmt.Printf("Part 1: %d\n", paper)
	fmt.Printf("Part 1: %d\n", ribbon)
}
