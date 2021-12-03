package main

import (
	"aoc/utils"
	"strconv"
)

func getInput(name string) []string {
	lines := utils.ReadLines(name, "\n")
	return lines
}

func counts(lines []string) []int {
	nums := make([]int, len(lines[0]))
	for _, line := range lines {
		for i, char := range line {
			n, err := strconv.Atoi(string(char))
			utils.Check(err)
			nums[i] += n
		}
	}
	return nums
}

func commonBits(lines []string, major string, minor string) string {
	nums := counts(lines)
	total := len(lines)
	binary := ""
	for _, sum := range nums {
		if sum*2 >= total {
			binary += major
		} else {
			binary += minor
		}
	}
	return binary
}

func decimal(s string) int {
	n, err := strconv.ParseInt(s, 2, 64)
	utils.Check(err)
	return int(n)
}

func p1(lines []string) int {
	gamma := commonBits(lines, "1", "0")
	epsilon := commonBits(lines, "0", "1")

	return decimal(gamma) * decimal(epsilon)
}

func filter(lines []string, index int, major string, minor string) []string {
	commonBits := commonBits(lines, major, minor)
	bit := string(commonBits[index])

	filtered := make([]string, 0)
	for _, line := range lines {
		if string(line[index]) == bit {
			filtered = append(filtered, line)
		}
	}

	if len(filtered) > 1 {
		return filter(filtered, index+1, major, minor)
	} else {
		return filtered
	}
}

func p2(lines []string) int {
	oxygen := filter(lines, 0, "1", "0")[0]
	co2 := filter(lines, 0, "0", "1")[0]
	return decimal(oxygen) * decimal(co2)
}

func main() {
	lines := getInput("./input.txt")

	println("p1:", p1(lines))
	println("p2:", p2(lines))
}
