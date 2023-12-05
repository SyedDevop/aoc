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

func parseCards(line string) (winingNumbers []string, myNumbers string) {
	cardsBody := strings.Split(line, ": ")
	cards := strings.Split(cardsBody[1], " |")
	winNumber := strings.Split(cards[0], " ")
	myNumber := cards[1] + " "

	return winNumber, myNumber
}

func sum(nums []int) int {
	numSum := 0
	for i := 0; i < len(nums); i++ {
		numSum = numSum + nums[i]
	}
	return numSum
}

func cardNumWone(numbers, n string) bool {
	num := strings.TrimSpace(n)
	if !isNumeric(num) {
		return false
	}
	if len(num) == 1 {
		num = " " + num + " "
	}
	return strings.Contains(numbers, num)
}

func defaultValueArray(length, value int) []int {
	s := make([]int, length)
	for i := range s {
		s[i] = value
	}
	return s
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

		winingNumbers, myNumbers := parseCards(line)

		for _, RawNum := range winingNumbers {
			isMatch := cardNumWone(myNumbers, RawNum)

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
	allRoundCount := defaultValueArray(len(input), 1)
	for carNum, cardValue := range input {
		winingNumbers, myNumbers := parseCards(cardValue)
		roundScore := 0
		for _, RawNum := range winingNumbers {
			isMatch := cardNumWone(myNumbers, RawNum)
			if isMatch {
				roundScore++
			}
		}
		for j := 1; j <= roundScore; j++ {
			allRoundCount[carNum+j] += allRoundCount[carNum]
		}
	}

	return sum(allRoundCount)
}

func parseInput(input string) []string {
	return strings.Split(input, "\n")
}
