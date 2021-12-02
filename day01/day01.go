package main

import (
	"os"
	"strconv"
	"strings"
)

func getInput(fname string) []int {
	data, err := os.ReadFile(fname)
	if err != nil {
		panic(err)
	}
	s := string(data)
	words := strings.Split(s, "\n")
	nums := make([]int, 0)
	for _, v := range words {
		num, err := strconv.Atoi(v)
		if err != nil {
			continue
		}
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
