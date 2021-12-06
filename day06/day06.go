package main

import (
	"aoc/utils"
	"fmt"
)

func p1(fishes []int, days int) {
	for day := 0; day < days; day++ {
		for i, fish := range fishes {
			fishes[i] -= 1
			if fish == 0 {
				fishes[i] = 6
				fishes = append(fishes, 8)
			}
		}
	}
	fmt.Printf("part 1: %d\n", len(fishes))
}

func p2(fishes []int, days int) {
	fishCounts := make([]int, 9)
	// populate initial state
	for _, fish := range fishes {
		fishCounts[fish] += 1
	}
	// shift counts for each growth stage group
	for day := 0; day < days; day++ {
		newFish := fishCounts[0]
		for i := 0; i < 7; i++ {
			fishCounts[i] = fishCounts[i+1]
		}
		fishCounts[6] += newFish
		fishCounts[8] = newFish
	}
	fmt.Printf("part 2: %d\n", utils.SumInt(fishCounts...))
}

func main() {
	fishesStr := utils.ReadLines("./input.txt", ",")

	fishes := make([]int, 0)
	for _, fishStr := range fishesStr {
		f := utils.StrToInt(fishStr)
		fishes = append(fishes, f)
	}

	fishesCopy := make([]int, len(fishes))
	copy(fishesCopy, fishes)

	p1(fishes, 80)
	p2(fishesCopy, 256)
}
