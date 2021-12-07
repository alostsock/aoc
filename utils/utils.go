package utils

import (
	"os"
	"strconv"
	"strings"
)

func Check(e error) {
	if e != nil {
		panic(e)
	}
}

func ReadLines(name string, delim string) []string {
	data, err := os.ReadFile(name)
	Check(err)
	lines := strings.Split(string(data), delim)
	notEmpty := make([]string, 0)
	for _, v := range lines {
		if len(strings.TrimSpace(v)) > 0 {
			notEmpty = append(notEmpty, v)
		}
	}
	return notEmpty
}

func StrToInt(s string) int {
	i, err := strconv.Atoi(s)
	Check(err)
	return i
}

func SumInt(nums ...int) int {
	sum := 0
	for _, n := range nums {
		sum += n
	}
	return sum
}

func MaxInt(nums ...int) int {
	max := 0
	for _, n := range nums {
		if n > max {
			max = n
		}
	}
	return max
}
