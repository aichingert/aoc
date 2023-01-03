// Advent of Code 2015, Day 4
// (c) aichingert

package main

import (
	"fmt"
	"os"
	"crypto/md5"
	"encoding/hex"
	"strings"
)

func GetMD5Hash(text string) string {
	hash := md5.Sum([]byte(text))
	return hex.EncodeToString(hash[:])
}

func Solve(key string, zeros int) int {
	n := 0

	for true {
		n += 1
		s := GetMD5Hash(fmt.Sprintf("%s%d", key, n))[:zeros]
		if strings.Count(s, "0") == zeros {
			return n
		}
	}

	return n
}

func main() {
	dat, err := os.ReadFile("../input/04")

	if err != nil {
		panic(err)
	}

	fmt.Printf("Part 1: %d\n", Solve(strings.TrimSpace(string(dat)),5))
	fmt.Printf("Part 2: %d\n", Solve(strings.TrimSpace(string(dat)),6))
}
