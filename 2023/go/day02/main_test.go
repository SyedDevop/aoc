package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestA(t *testing.T) {
	// 	input := `Game 2: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
	// Game 3: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
	// Game 4: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
	// Game 5: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
	// Game 6: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`

	result := part1(parseInput(input))

	require.Equal(t, 2593, result)
}

func BenchmarkA(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = part1(parseInput(input))
	}
}

func TestB(t *testing.T) {
	// 	input := `Game 2: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
	// Game 3: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
	// Game 4: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
	// Game 5: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
	// Game 6: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`

	result := part2(parseInput(input))

	require.Equal(t, 54699, result)
}

func BenchmarkB(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = part2(parseInput(input))
	}
}
