package main

import (
	_ "embed"
	"fmt"
	"strings"
	"unicode"
)

//go:embed input
var input string

func init() {
	input = strings.TrimRight(input, "\n")
	if len(input) == 0 {
		panic("empty input file")
	}
}

func main() {
	part1 := part1(parseInput(input))
	fmt.Println(part1)
}

func part1(input []string) int {
	// var nums []string
	sum := 0

	for _, v := range input {
		first := -1
		last := -1
		for _, a := range v {
			if unicode.IsNumber(a) {
				if first == -1 {
					first = (int(a) - '0')
					// last = int(a)
				}
				last = (int(a) - '0')

			}
		}

		sum += (first * 10) + last
	}
	return sum
}
func part2(input []string) int { return 0 }

func parseInput(input string) []string {
	return strings.Split(input, "\n")
}
