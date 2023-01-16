// Advent of Code 2015, day 7
// (c) aichingert

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

func solve(cond map[string]string, vals map[string]int, cur string) int {
	res, err := strconv.Atoi(cur)
	
	if err == nil {
		return res
	}

	val, ok := vals[cur]

	if ok {
		return val
	}

	s := strings.Split(cond[cur], " ")
	ret := 0

	switch len(s) {
	case 1:
		ret = solve(cond, vals, s[0])
	case 2:
		ret = ^solve(cond, vals, s[1])
	case 3:
		switch s[1] {
		case "AND":
			ret = solve(cond, vals, s[0]) & solve(cond, vals, s[2])
		case "OR":
			ret = solve(cond, vals, s[0]) | solve(cond, vals, s[2])
		case "LSHIFT":
			shift,_ := strconv.Atoi(s[2])
			ret = solve(cond, vals, s[0]) << shift
		case "RSHIFT":
			shift,_ := strconv.Atoi(s[2])
			ret = solve(cond, vals, s[0]) >> shift
		}
	}

	vals[cur] = ret
	return ret
}

func parse(lines []string) map[string]string {
	vals := make(map[string]string)

	for i := range lines {
		splt := strings.Split(lines[i], " -> ")
		vals[splt[1]] = splt[0]
	}

	return vals
}

func main() {
	dat, err := os.ReadFile("../input/07")

	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.ReplaceAll(string(dat), "\r\n", "\n"), "\n")
	cond := parse(lines)
	vals := make(map[string]int)

	a := solve(cond, vals, "a")
	for k := range vals { delete(vals,k) }
	vals["b"] = a

	fmt.Printf("Part 1: %d\n", a)
	fmt.Printf("Part 2: %d\n", solve(cond, vals, "a"))
}