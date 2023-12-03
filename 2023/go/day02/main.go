package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

//go:embed input
var input string

var maxSube = map[string]int{
	"red":   12,
	"green": 13,
	"blue":  14,
}

func init() {
	input = strings.TrimRight(input, "\n")
	if len(input) == 0 {
		panic("empty input file")
	}
}

func main() {
	part1 := part1(parseInput(input))
	part2 := part2(parseInput(input))
	fmt.Printf("Part1 answer: %d\nPart2 answer: %d\n", part1, part2)
}

func part1(input []string) int {
	sumRound := 0
	for i, round := range input {
		sumIt := true
		for _, sets := range strings.Split(round[8:], "; ") {
			for _, sube := range strings.Split(sets, ", ") {
				word := strings.Split(sube, " ")
				x, _ := strconv.Atoi(string(word[0]))
				colorName := word[1]

				maxNum := maxSube[colorName]
				if x > maxNum {
					sumIt = false
				}
			}
		}
		if sumIt {
			sumRound += (i + 1)
		}

	}

	return sumRound
}

func part2(input []string) int {
	sumRound := 0

	for _, round := range input {
		maxSube := map[string]int{
			"red":   0,
			"green": 0,
			"blue":  0,
		}
		body := strings.Split(round, ": ")
		for _, sets := range strings.Split(body[1], "; ") {
			for _, sube := range strings.Split(sets, ", ") {
				word := strings.Split(sube, " ")
				x, _ := strconv.Atoi(string(word[0]))
				colorName := word[1]

				preNum := maxSube[colorName]
				if x > preNum {
					maxSube[colorName] = x
				}

			}
		}
		sumRound += (maxSube["red"] * maxSube["green"] * maxSube["blue"])
	}
	return sumRound
}

func parseInput(input string) []string {
	return strings.Split(input, "\n")
}
