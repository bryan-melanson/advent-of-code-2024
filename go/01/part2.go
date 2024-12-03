package main

import (
	"log"
	"sort"
	"strconv"
	"strings"
)

func solve2(input string) int {
	result := 0
	var leftNumbers, rightNumbers []int

	lines, err := readLines(input)
	if err != nil {
		log.Fatal(err)
	}

	for _, line := range lines {
		numbers := strings.Fields(line)

		left, _ := strconv.Atoi(numbers[0])
		right, _ := strconv.Atoi(numbers[1])

		leftNumbers = append(leftNumbers, left)
        rightNumbers = append(rightNumbers, right)
	}

	sort.Ints(leftNumbers)
    sort.Ints(rightNumbers)

	for i := 0; i < len(leftNumbers); i++ {
		j := 0
		for rightNumbers[j] < leftNumbers[i] {
			j++
		}
		for rightNumbers[j] == leftNumbers[i] {
			result += leftNumbers[i]
			j++
		}
    }

	return result
}
