package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestA(t *testing.T) {
	// 	input := `Card  76: 70 94  3  1 46 48 87  5 16 74 | 52 14 22  6 24 65  4  8 42 36 66 43  9 45 93 69 51 57 19 44 81 98 77 35 79
	// Card  77: 77  8 29 21 11 31 93 74 72 71 | 67  8 43 72 62 40 11 77 71 29 61 92 74 12 52 37 78 93 56 31 14 21 63 39 35
	// Card  78: 66 48  5  4 63 54 91 74 76 77 | 48 54 36 95 11 61 76 52 46 65 18 67 66 63 62 19  5 74 77 64  4 42  9 91 55`

	result := part1(parseInput(input))

	require.Equal(t, 23235, result)
}

func BenchmarkA(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = part1(parseInput(input))
	}
}

func TestB(t *testing.T) {
	// 	input := `Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
	// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
	// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
	// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
	// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
	// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11`

	result := part2(parseInput(input))

	require.Equal(t, 5883416, result)
}

func BenchmarkB(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = part2(parseInput(input))
	}
}
