package main

import (
	"aoc/utils"
	"strconv"
)

func getInput(name string) []int {
	lines := utils.ReadLines(name, "\n")
	nums := make([]int, 0)
	for _, v := range lines {
		num, err := strconv.Atoi(v)
		utils.Check(err)
		nums = append(nums, num)
	}
	return nums
}

func p1(nums []int) int {
	prev := 0
	increases := 0
	for i, v := range nums {
		if i > 0 && v > prev {
			increases += 1
		}
		prev = v
	}
	return increases
}

func p2(nums []int) int {
	prevSum := 0
	increases := 0
	for i := range nums {
		if i == 0 || i == len(nums)-1 {
			continue
		}
		sum := nums[i-1] + nums[i] + nums[i+1]
		if i > 1 && sum > prevSum {
			increases += 1
		}
		prevSum = sum
	}
	return increases
}

func main() {
	nums := getInput("./input.txt")

	println("p1:", p1(nums))
	println("p2:", p2(nums))
}
