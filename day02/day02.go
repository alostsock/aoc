package main

import (
	"aoc/utils"
	"strconv"
	"strings"
)

type move struct {
	dir  string
	dist int
}

type point struct {
	x int
	y int
}

func getMove(line string) move {
	x := strings.Split(line, " ")
	dir := x[0]
	dist, err := strconv.Atoi(x[1])
	utils.Check(err)
	return move{dir, dist}
}

func getInput(name string) []move {
	lines := utils.ReadLines(name, "\n")
	moves := make([]move, 0)
	for _, line := range lines {
		moves = append(moves, getMove(line))
	}
	return moves
}

func p1(moves []move) int {
	p := point{0, 0}
	for _, move := range moves {
		if move.dir == "down" {
			p.y += move.dist
		} else if move.dir == "up" {
			p.y -= move.dist
		} else {
			p.x += move.dist
		}
	}
	return p.x * p.y
}

func p2(moves []move) int {
	a := 0
	p := point{0, 0}
	for _, move := range moves {
		if move.dir == "down" {
			a += move.dist
		} else if move.dir == "up" {
			a -= move.dist
		} else {
			p.x += move.dist
			p.y += a * move.dist
		}
	}
	return p.x * p.y
}

func main() {
	moves := getInput("./input.txt")

	println("p1:", p1(moves))
	println("p2:", p2(moves))
}
