package main

import (
	"aoc/utils"
	"math"
)

type cache map[int]int

func distanceCache(positions []int) cache {
	maxDist := utils.MaxInt(positions...)
	c := make(cache)
	prevDist := 0
	for step := 0; step <= maxDist; step++ {
		dist := prevDist + step
		c[step] = dist
		prevDist = dist
	}
	return c
}

func minFuel(positions []int, isSimple bool) int {
	minFuel := math.MaxInt
	cache := distanceCache(positions)
	for guess := 0; guess < len(positions); guess++ {
		fuel := 0
		for _, n := range positions {
			distance := int(math.Abs(float64(n - guess)))
			if isSimple {
				fuel += distance
			} else {
				fuel += cache[distance]
			}
		}
		if fuel < minFuel {
			minFuel = fuel
		}
	}
	return minFuel
}

func main() {
	lines := utils.ReadLines("./input.txt", ",")
	crabPositions := make([]int, 0)
	for _, line := range lines {
		crabPositions = append(crabPositions, utils.StrToInt(line))
	}

	println("part 1:", minFuel(crabPositions, true))
	println("part 2:", minFuel(crabPositions, false))
}
