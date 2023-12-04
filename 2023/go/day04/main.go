package main

import (
	_ "embed"
	"fmt"
	"strconv"
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

func isNumeric(s string) bool {
	_, err := strconv.Atoi(s)
	return err == nil
}

func main() {
	part1 := part1(parseInput(input))
	part2 := part2(parseInput(input))
	fmt.Printf("Part1 answer: %d\nPart2 answer: %d\n", part1, part2)
}

func part1(input []string) int {
	sumRound := 0
	for _, line := range input {
		roungScore := 0
		cardsBody := strings.Split(line, ": ")

		cards := strings.Split(cardsBody[1], " |")
		winNumber := strings.Split(cards[0], " ")
		myNumber := cards[1] + " "

		for _, RawNum := range winNumber {
			num := strings.TrimSpace(RawNum)
			if !isNumeric(num) {
				continue
			}
			if len(num) == 1 {
				num = " " + num + " "
			}
			isMatch := strings.Contains(myNumber, num)

			if isMatch {
				if roungScore == 0 {
					roungScore = 1
				} else {
					roungScore *= 2
				}
			}
		}
		sumRound += roungScore
	}
	return sumRound
}

func part2(input []string) int {
	return 0
}

func parseInput(input string) []string {
	return strings.Split(input, "\n")
}
