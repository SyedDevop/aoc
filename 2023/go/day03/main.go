package main

import (
	_ "embed"
	"fmt"
	"strings"
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
	// part2 := part2(parseInput(input))
	part2 := 0
	fmt.Printf("Part1 answer: %d\nPart2 answer: %d\n", part1, part2)
}

func part1(input []string) int {
	sumRound := 0

	return sumRound
}

func part2(input []string) int {
	sumRound := 0

	return sumRound
}

func parseInput(input string) []string {
	return strings.Split(input, "\n")
}
