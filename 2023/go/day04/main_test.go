package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestA(t *testing.T) {
	input := `Card  76: 70 94  3  1 46 48 87  5 16 74 | 52 14 22  6 24 65  4  8 42 36 66 43  9 45 93 69 51 57 19 44 81 98 77 35 79
Card  77: 77  8 29 21 11 31 93 74 72 71 | 67  8 43 72 62 40 11 77 71 29 61 92 74 12 52 37 78 93 56 31 14 21 63 39 35
Card  78: 66 48  5  4 63 54 91 74 76 77 | 48 54 36 95 11 61 76 52 46 65 18 67 66 63 62 19  5 74 77 64  4 42  9 91 55`

	result := part1(parseInput(input))

	require.Equal(t, 1024, result)
}

func BenchmarkA(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = part1(parseInput(input))
	}
}

func TestB(t *testing.T) {
	input := `Game 2: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
	Game 3: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
	Game 4: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
	Game 5: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
	Game 6: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`

	result := part2(parseInput(input))

	require.Equal(t, 0, result)
}

func BenchmarkB(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = part2(parseInput(input))
	}
}
