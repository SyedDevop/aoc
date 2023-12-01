package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"
	"unicode"
)

//go:embed input
var input string

func init() {
	input = strings.TrimRight(input, "\n")
	if len(input) == 0 {
		panic("empty input.txt file")
	}
}

func main() {
	part1 := part1(parseInput(input))
	fmt.Println(part1)
}

func part1(input []string) int {
	var nums []string
	sum := 0
	for x, v := range input {
		for _, a := range v {
			if unicode.IsNumber(a) {
				nums = append(nums, string(a))
				break
			}
		}

		for i := len(v) - 1; i >= 0; i-- {
			curChar := v[i]
			if unicode.IsNumber(rune(curChar)) {
				nums[x] += string(v[i])
				break
			}
		}
	}

	for _, num := range nums {
		if intVar, err := strconv.Atoi(num); err == nil {
			sum += intVar
		}
	}

	return sum
}
func part2(input []string) int { return 0 }

func parseInput(input string) []string {
	return strings.Split(input, "\n")
}
