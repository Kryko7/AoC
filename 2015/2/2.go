package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"slices"
)

func parseFile(filename string) [][]int {
	file, err := os.Open(filename)
	if err != nil {
		fmt.Printf("Failed to open file: %v", err)
	}
	defer file.Close()

	var result [][]int
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text();
		parts := strings.Split(line, "x")
		var dims []int
		for _, part := range parts {
			num, err := strconv.ParseInt(strings.TrimSpace(part), 10 ,32)
			if err != nil {
				fmt.Println("Failed to parse line!")
			}
			dims = append(dims, int(num))
		}
		result = append(result, dims)
	}
	return result;
}

func wrappingPapers(intput [][]int) int {
	total := 0
	for _, dims := range intput {
		wrapping := 2 * dims[0] * dims[1] + 2 * dims[1] * dims[2] + 2 * dims[0] * dims[2]
		slices.Sort(dims)
		slack := dims[0] * dims[1]
		total += wrapping + slack
	}
	return total
}

func ribbons(input [][]int) int {
	total := 0
	for _, dims := range input {
		wrapping := dims[0] * dims[1] * dims[2]
		slices.Sort(dims)
		slack := 2 * dims[0] + 2 * dims[1]
		total += wrapping + slack
	}
	return total
}

func main() {
	filename := "2.txt"
	input := parseFile(filename)
	ans1 := wrappingPapers(input)
	ans2 := ribbons(input)
	fmt.Println("Answer for part1:", ans1)
	fmt.Println("Answer for part2:", ans2)
}