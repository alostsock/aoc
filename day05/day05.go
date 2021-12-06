package main

import (
	"aoc/utils"
	"fmt"
	"strings"
)

type diagram map[[2]int]int

type point struct {
	x int
	y int
}

type segment struct {
	from point
	to   point
}

func getSegment(line string) segment {
	// line: "0,9 => 5,9"
	parts := strings.Split(line, " ")
	p0 := strings.Split(parts[0], ",")
	p1 := strings.Split(parts[2], ",")
	seg := segment{}
	seg.from.x = utils.StrToInt(p0[0])
	seg.from.y = utils.StrToInt(p0[1])
	seg.to.x = utils.StrToInt(p1[0])
	seg.to.y = utils.StrToInt(p1[1])
	return seg
}

func getDirection(start int, end int) int {
	if end > start {
		return 1
	} else if end < start {
		return -1
	} else {
		return 0
	}
}

func drawLineSegment(seg segment, d diagram, ignoreDiagonals bool) diagram {
	xDir := getDirection(seg.from.x, seg.to.x)
	yDir := getDirection(seg.from.y, seg.to.y)
	if ignoreDiagonals && seg.from.x != seg.to.x && seg.from.y != seg.to.y {
		return d
	}
	x := seg.from.x
	y := seg.from.y
	continueDrawing := true
	for continueDrawing {
		d[[2]int{x, y}] += 1
		continueDrawing = x != seg.to.x || y != seg.to.y
		x += xDir
		y += yDir
	}
	return d
}

func countIntersections(d diagram) int {
	sum := 0
	for _, v := range d {
		if v >= 2 {
			sum += 1
		}
	}
	return sum
}

func p1(lines []string) {
	d := make(diagram)
	for _, line := range lines {
		seg := getSegment(line)
		drawLineSegment(seg, d, true)
	}
	fmt.Printf("part 1: %d\n", countIntersections(d))
}

func p2(lines []string) {
	d := make(diagram)
	for _, line := range lines {
		seg := getSegment(line)
		drawLineSegment(seg, d, false)
	}
	fmt.Printf("part 2: %d\n", countIntersections(d))
}

func main() {
	lines := utils.ReadLines("./input.txt", "\n")

	p1(lines)
	p2(lines)
}
