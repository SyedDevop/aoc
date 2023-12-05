package main

import "testing"

func BenchmarkA(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = part1(parseInput(input))
	}
}

func BenchmarkB(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = part2(parseInput(input))
	}
}
