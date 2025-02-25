package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

 func parse(path string) []int64 {
	file, err := os.Open(path)
	if err != nil {
		log.Fatal("Error occured while loading the file", err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	var stones []int64
	for scanner.Scan() {
		text := scanner.Text()
		values := strings.Split(text, " ")
		for _, val := range values {
			i, err := strconv.ParseInt(val, 10, 64)
			if err != nil {
				log.Fatal("Error ocurred while transforming string -> int", err)
			}
			stones = append(stones, i)
		}
	}
	return stones
 }

 type Even struct {
	even bool
	size int
}

 func isEven(stone int64) Even {
	i := 0
	for stone > 0 {
		i += 1
		stone /= 10
	}
	return Even{i % 2 == 0, i}
 }

 type Partitions struct {
	left int64
	right int64
 }

 func partition(stone, size int64) Partitions {
	div := math.Pow(10, float64(size) / 2)
	left := stone / int64(div)
	right := stone % int64(div)
	return Partitions{left, right}
 }

 func blink(times int64, stones []int64) int64 {
	
 }

 func main() {
	stones := parse("/home/minindu/AoC/2024/11/11.txt")
	fmt.Println("Stones: ", stones)
 }