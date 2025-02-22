package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Equation struct {
	index    int
	target   uint64
	equation []uint64
}

func parseFile(filename string) []Equation {
	equations := make([]Equation, 0)
	file, err := os.Open(filename)
	if err != nil {
		log.Fatalf("Failed to open file: %v", err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	lineNumber := 0
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Split(line, ":")
		if len(parts) != 2 {
			log.Fatalf("Invalid line format at line %d: %s", lineNumber, line)
		}
		target, err := strconv.ParseInt(parts[0], 10, 64)
		if err != nil {
			log.Fatalf("Failed to parse target at line %d: %v", lineNumber, err)
		}
		equationParts := strings.Fields(parts[1])
		equation := make([]uint64, len(equationParts))
		for i, part := range equationParts {
			parsedInt, err := strconv.ParseInt(part, 10, 64)
			if err != nil {
				log.Fatalf("Failed to parse equation part at line %d: %v", lineNumber, err)
			}
			equation[i] = uint64(parsedInt)
			if err != nil {
				log.Fatalf("Failed to parse equation part at line %d: %v", lineNumber, err)
			}
		}
		equations = append(equations, Equation{
			index:    lineNumber,
			target:   uint64(target),
			equation: equation,
		})
		lineNumber++
	}
	return equations
}

func solveEquation(equation []uint64, index int, target uint64, currSum uint64) bool {
	if (index == len(equation)) {
		return currSum == target
	}
	return solveEquation(equation, index + 1, target, currSum + equation[index]) ||
		solveEquation(equation, index + 1, target, currSum * equation[index])
}

func main() {
	fmt.Println("Hello, playground.")
	equations := parseFile("AoC2024_7.txt")
	var total uint64 = 0
	for _, equation := range equations {
		if solveEquation(equation.equation, 0, equation.target, 0) {
			total += equation.target
		}
	} 
	fmt.Println("Total: ", total)
}
