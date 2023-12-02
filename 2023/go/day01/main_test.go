package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestA(t *testing.T) {
	// 	input := `1abc2
	// pqr3stu8vwx
	// a1b2c3d4e5f
	// treb7uchet`
	//
	result := part1(parseInput(input))

	require.Equal(t, 53194, result)
}

func BenchmarkA(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = part1(parseInput(input))
	}
}
