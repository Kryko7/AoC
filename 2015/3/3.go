package main

import (
	"bufio"
	"fmt"
	"os"
)

type Location struct {
	X int
	Y int
}
func parseFile(filename string) []byte {
	file, err := os.Open(filename);
	if err != nil {
		fmt.Println("Error reading file.")
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var input []byte
	for scanner.Scan() {
		line := scanner.Text()
		input = []byte(line)
	}
	return input
}


func uniqueHouses(input []byte) int {
	locations := make(map[Location]struct{})
	x := 0
	y := 0
	initLocation := Location{X: x, Y: y}
	locations[initLocation] = struct{}{}
	for _, c := range input {
		switch c {
			case '>':
				x++
			case '<':
				x--
			case '^':
				y++
			default:
				y--
		} 
		if _, exists := locations[Location{X: x, Y: y}]; !exists {
			locations[Location{X: x, Y: y}] = struct{}{}
		}
	}
	return len(locations)
}

func uniqueHouses2(input []byte) int {
	locations := make(map[Location]struct{})
	x := 0
	y := 0
	a := 0
	b := 0
	initLocation := Location{X: x, Y: y}
	locations[initLocation] = struct{}{}
	for i, c := range input {
		if i % 2 == 0 {
			switch c {
				case '>':
					x++
				case '<':
					x--
				case '^':
					y++
				default:
					y--
			} 
			if _, exists := locations[Location{X: x, Y: y}]; !exists {
				locations[Location{X: x, Y: y}] = struct{}{}
			}
		} else {
			switch c {
				case '>':
					a--
				case '<':
					a++
				case '^':
					b--
				default:
					b++
			}
			if _, exists := locations[Location{X: a, Y: b}]; !exists {
				locations[Location{X: a, Y: b}] = struct{}{}
			}
		}
	}
	return len(locations)
}

func main() {
	filename := "3.txt"
	input := parseFile(filename)
	ans1 := uniqueHouses(input)
	ans2 := uniqueHouses2(input)
	fmt.Println("Answer for the part 1:", ans1)
	fmt.Println("Answer for the part 2:", ans2)
}