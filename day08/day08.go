package main

import (
	"aoc/utils"
	"strconv"
	"strings"
)

type entry struct {
	signal10 []string
	signal4  []string
}

func split(s string, delim string) []string {
	trimmed := strings.TrimSpace(s)
	return strings.Split(trimmed, delim)
}

func getEntries(lines []string) []entry {
	entries := make([]entry, len(lines))
	for i, line := range lines {
		parts := split(line, "|")
		entries[i].signal10 = split(parts[0], " ")
		entries[i].signal4 = split(parts[1], " ")
	}
	return entries
}

func countUniqueSignals(e entry) int {
	sum := 0
	for _, signal := range e.signal4 {
		// 2 = one, 4 = four, 3 = seven, 7 = eight
		switch len(signal) {
		case 2, 4, 3, 7:
			sum += 1
		}
	}

	return sum
}

func p1(entries []entry) {
	sum := 0
	for _, entry := range entries {
		sum += countUniqueSignals(entry)
	}
	println(sum)
}

func mapSignal(letters string) [7]bool {
	var signal [7]bool
	place := map[string]int{"a": 0, "b": 1, "c": 2, "d": 3, "e": 4, "f": 5, "g": 6}
	for _, letter := range letters {
		signal[place[string(letter)]] = true
	}
	return signal
}

func diff(a [7]bool, b [7]bool) int {
	difference := 0
	for i := 0; i < 7; i++ {
		if a[i] != b[i] {
			difference += 1
		}
	}
	return difference
}

// numbers one, four, seven, eight have signal length 2, 4, 3, 7 respectively
//
// this leaves zero, two, three, five, six, nine.
//
// six: length 6 and 5 away from seven
// nine: length 6 and 2 away from four
// zero: neither six or nine
//
// two:  length 5 and 5 away from four
// three: length 5 and 2 away from seven
// five: neither two or three
func decodeEntry(entry entry) int {
	signalLookup := make(map[[7]bool]int)
	// decode unique signals
	uniqueLookup := map[int]int{2: 1, 4: 4, 3: 7, 7: 8}
	uniques := make(map[int][7]bool)
	for _, letters := range entry.signal10 {
		if num, ok := uniqueLookup[len(letters)]; ok {
			signal := mapSignal(letters)
			signalLookup[signal] = num
			uniques[num] = signal
		}
	}
	// everything else
	for _, letters := range entry.signal10 {
		signal := mapSignal(letters)
		if len(letters) == 6 {
			if diff(signal, uniques[7]) == 5 {
				signalLookup[signal] = 6
			} else if diff(signal, uniques[4]) == 2 {
				signalLookup[signal] = 9
			} else {
				signalLookup[signal] = 0
			}
		} else if len(letters) == 5 {
			if diff(signal, uniques[4]) == 5 {
				signalLookup[signal] = 2
			} else if diff(signal, uniques[7]) == 2 {
				signalLookup[signal] = 3
			} else {
				signalLookup[signal] = 5
			}
		}
	}

	result := ""
	for _, letters := range entry.signal4 {
		signal := mapSignal(letters)
		digit := strconv.Itoa(signalLookup[signal])
		result += digit
	}
	return utils.StrToInt(result)
}

func p2(entries []entry) {
	sum := 0
	for _, entry := range entries {
		result := decodeEntry(entry)
		sum += result
	}
	println(sum)
}

func main() {
	lines := utils.ReadLines("./input.txt", "\n")
	entries1 := getEntries(lines)

	entries2 := make([]entry, len(entries1))
	copy(entries2, entries1)

	p1(entries1)
	p2(entries2)
}
