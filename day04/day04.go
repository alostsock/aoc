package main

import (
	"aoc/utils"
	"strconv"
	"strings"
)

type input struct {
	draws  []int
	boards []board
}

type board struct {
	positions map[position]int
	numbers   map[int]position
	points    map[position]bool
}

type position [2]int

func getInput(name string) input {
	raw := utils.ReadLines(name, "\n\n")
	draws := make([]int, 0)
	for _, drawStr := range strings.Split(raw[0], ",") {
		draw, err := strconv.Atoi(drawStr)
		utils.Check(err)
		draws = append(draws, draw)
	}
	boards := make([]board, 0)
	for _, boardStr := range raw[1:] {
		boards = append(boards, stringToboard((boardStr)))
	}
	return input{draws, boards}
}

func stringToboard(bStr string) board {
	b := board{
		positions: make(map[position]int),
		numbers:   make(map[int]position),
		points:    make(map[position]bool),
	}
	for m, line := range strings.Split(bStr, "\n") {
		for n, start := range []int{0, 3, 6, 9, 12} {
			numStr := strings.TrimSpace(string(line[start : start+2]))
			num, err := strconv.Atoi(numStr)
			utils.Check(err)
			pos := position{n, m}
			b.positions[pos] = num
			b.numbers[num] = pos
		}
	}
	return b
}

func checkDimension(b board, isRow bool) bool {
	dim := [5]int{0, 1, 2, 3, 4}
	for _, n := range dim {
		rowBingo := true
		for _, m := range dim {
			var pos position
			if isRow {
				pos = position{n, m}
			} else {
				pos = position{m, n}
			}
			_, rowBingo = b.points[pos]
			if !rowBingo {
				break
			}
		}
		if rowBingo {
			return true
		}
	}
	return false
}

func sumUnmarked(b board) int {
	sum := 0
	for pos, num := range b.positions {
		_, marked := b.points[pos]
		if !marked {
			sum += num
		}
	}
	return sum
}

func hasBingo(b board) bool {
	if checkDimension(b, true) {
		return true
	}
	if checkDimension(b, false) {
		return true
	}
	return false
}

func p1(in input) int {
	for _, draw := range in.draws {
		for _, board := range in.boards {
			if pos, ok := board.numbers[draw]; ok {
				board.points[pos] = true
			}
			if hasBingo(board) {
				return sumUnmarked(board) * draw
			}
		}
	}
	return 0
}

func p2(in input) int {
	wonScores := make(map[int]int)
	lastWonIndex := 0
	for _, draw := range in.draws {
		for boardIndex, board := range in.boards {
			if _, hasWon := wonScores[boardIndex]; hasWon {
				continue
			}
			if pos, ok := board.numbers[draw]; ok {
				board.points[pos] = true
			}
			if hasBingo(board) {
				score := sumUnmarked(board) * draw
				wonScores[boardIndex] = score
				lastWonIndex = boardIndex
			}
		}
	}
	return wonScores[lastWonIndex]
}

func main() {
	in := getInput("./input.txt")

	println("part 1:", p1(in))
	println("part 2:", p2(in))
}
