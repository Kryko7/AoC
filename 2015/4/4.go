package main

import (
	"bufio"
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"os"
	"strconv"
)


func parseFile(filename string) string {
	file, err := os.Open(filename);
	if err != nil {
		fmt.Println("Error reading file.")
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var input string
	for scanner.Scan() {
		line := scanner.Text()
		input = line
	}
	return input

}


func adventCoin5(input string) int {
	i := 1
	found := false
	for !found {
		s := input + strconv.Itoa(i)
		md5Hash := md5.Sum([]byte(s))
		hashString := hex.EncodeToString(md5Hash[:])
		if hashString[0:5] == "00000" {
			found = true
		}
		i++
	}
	return i-1
}

func adventCoin6(input string) int {
	i := 1
	found := false
	for !found {
		s := input + strconv.Itoa(i)
		md5Hash := md5.Sum([]byte(s))
		hashString := hex.EncodeToString(md5Hash[:])
		if hashString[0:6] == "000000" {
			found = true
		}
		i++
	}
	return i-1
}

func main() {
	filename := "4.txt"
	input := parseFile(filename)
	ans1 := adventCoin5(input)
	ans2 := adventCoin6(input)
	fmt.Println("Answer for part 1:", ans1)
	fmt.Println("Answer for part 2:", ans2)
}
