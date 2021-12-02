package utils

import (
	"os"
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
