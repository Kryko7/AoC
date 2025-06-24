package main

import (
	"fmt"
	"os"
)

func parseFile(filename string) string{
	file, err := os.ReadFile(filename)
	if err != nil {
		fmt.Printf("Failed to open file: %v", err)
	}

	text := string(file)
	return text
}

func checkFloor(text string) int {
	floor := 0
	for _, char := range text {
		if char == '(' {
			floor++
		} else {
			floor--
		}
	}
	return floor
}

func firstChance(text string) int {
	floor := 0
	for pos, char := range text {
		if char == '(' {
			floor++
		} else {
			floor--
			if floor == -1 {
				return pos + 1
			}
		}
	}
	return -1
}

func main() {
	filename := "1.txt"
	text := parseFile(filename)
	ans1 := checkFloor(text)
	ans2 := firstChance(text)
	fmt.Println("Answer for part 1: ", ans1)
	fmt.Println("Answer for part 2: ", ans2)
}	