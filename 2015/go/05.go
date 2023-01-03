// Advent of Code 2015, Day 5
// (c) aichingert

package main

import (
	"fmt"
	"os"
	"strings"
)

// You can't create a const array in go :I
var I = []string{"ab", "cd", "pq", "xy"}

func part1(lines []string) int {
	c := 0

	for _,s := range lines {
		vc := 0
		if is_vowel(s[len(s)-1]) { vc += 1 }
		v := true
		dd := false

		for i:=0 ; i < len(s)-1 ; i++ {
			if is_vowel(s[i]) { vc += 1 }
			if s[i] == s[i+1] { dd = true }
			if contains(string(s[i])+string(s[i+1])) {
				v = false
				break
			}
		}

		if v && dd && vc > 2 { c += 1 }
	}

	return c
}

func is_vowel(ch byte) bool {
	return ch == 97 || ch == 101 || ch == 105 || ch == 111 || ch == 117
}

func contains(ch string) bool {
	for i:=0 ; i < len(I) ; i++ {
		if ch == I[i] { return true }
	}
	return false
}

func part2(lines []string) int {
	c := 0

	for _,s := range lines {
		f := false
		t := false

		for i := 0 ; i < len(s)-2 ; i++ {
			for j := i+2 ; j < len(s)-1 ; j++ {
				if s[i] == s[j] && s[i+1] == s[j+1] { f = true }
			}

			if s[i] == s[i+2] { t = true }
		}

		if f && t { c += 1 }
	}

	return c
}

func main() {
	dat, err := os.ReadFile("../input/05")

	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(dat), "\n")
	fmt.Printf("Part 1: %d\n", part1(lines))
	fmt.Printf("Part 2: %d\n", part2(lines))
}
