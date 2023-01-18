// Advent of Code 2016, day 2
// (c) aichingert

package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

var F = [3][3]int{{1,2,3},{4,5,6},{7,8,9}}
var S = [5][5]string{{".",".","1",".","."}, {".","2","3","4","."}, {"5","6","7","8","9"},{".","A","B","C","."}, {".",".","D",".","."}}

func part1(lines []string) string {
	ans := ""
	loc := [2]int{1,1}

	for i := range lines {
		for j := range lines[i] {
			switch lines[i][j] {
			case 85:
				loc[1] = max(0, loc[1]-1)
			case 76:
				loc[0] = max(0, loc[0]-1)
			case 82:
				loc[0] = min(2, loc[0]+1)
			case 68:
				loc[1] = min(2, loc[1]+1)
			}
		}
		ans += strconv.Itoa(F[loc[1]][loc[0]])
	}
	return ans
}

func min(a int, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a int, b int) int {
	if a > b {
		return a
	}
	return b
}

func part2(lines []string) string {
	ans := ""
	loc := [2]int{0,2}

	for i := range lines {
		for j := range lines[i] {
			switch lines[i][j] {
			case 85:
				if loc[1]>0 && S[loc[1]-1][loc[0]] != "." { loc[1] -= 1 }
			case 76:
				if loc[0]>0 && S[loc[1]][loc[0]-1] != "." { loc[0] -= 1 }
			case 82:
				if loc[0]+1<len(S) && S[loc[1]][loc[0]+1] != "." { loc[0] += 1 }
			case 68:
				if loc[1]+1<len(S) && S[loc[1]+1][loc[0]] != "." { loc[1] += 1 }
			}
		}
		ans += S[loc[1]][loc[0]]
	}

	return ans
}

func main() {
	dat, err := os.ReadFile("../input/02")

	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.ReplaceAll(string(dat), "\r\n", "\n"), "\n")
	fmt.Printf("Part 1: %s\n", part1(lines))
	fmt.Printf("Part 2: %s\n", part2(lines))
}
