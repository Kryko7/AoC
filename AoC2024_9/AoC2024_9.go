
package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func parsing(filename string) string{
	file, err := os.Open(filename)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var text string
	for scanner.Scan() {
		text = scanner.Text()
	}
	return text
}

func mapping(text string) []int{
	var numbers []int
	index := 0
	for i, v := range text {
		n := int(v - '0')
		if i % 2 == 0 {
			for j:=0; j<n; j++ {
				numbers = append(numbers, index)
			}
			index++
		} else {
			for j:=0; j<n; j++ {
				numbers = append(numbers, -1)
			}
		}
	}
	return numbers
}

func swapping(numbers []int) []int{
	left := 0
	right := len(numbers)-1
	for left < right {
		for left < right && numbers[left] != -1 {
			left++
		}
		for left < right && numbers[right] == -1 {
			right--
		}
		if left < right {
			numbers[left], numbers[right] = numbers[right], numbers[left]
		}
	}
	return numbers
}

func calculateChecksum(numbers []int) int{
	checksum := 0
	index := 0
	for index < len(numbers) && numbers[index] != -1 {
		checksum += numbers[index] * index
		index++
	}
	return checksum
}

func main() {
	text:=parsing("AoC2024_9.txt")
	result:=mapping(text)
	result=swapping(result)
	checksum:=calculateChecksum(result)
	fmt.Println("Checksum:", checksum)
}