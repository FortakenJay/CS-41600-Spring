package sorts

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func ReadInput() []int {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)
	var values []int

	for scanner.Scan() {
		word := scanner.Text()
		if word == "" {
			continue
		}

		num, err := strconv.Atoi(word)
		if err != nil {
			continue
		}

		values = append(values, num)
	}

	return values
}

func PrintOutput(s []int) {
	for _, num := range s {
		fmt.Println(num)
	}
}

func BubbleSort(s []int) {
	n := len(s)
	for i := 0; i < n-1; i++ {
		for j := 0; j < n-i-1; j++ {
			if s[j] > s[j+1] {
				s[j], s[j+1] = s[j+1], s[j]
			}
		}
	}
}

func MergeSort(s []int) {
	if len(s) <= 1 {
		return
	}
	mid := len(s) / 2
	MergeSort(s[:mid])
	MergeSort(s[mid:])

	tmp := make([]int, len(s))
	i, j, k := 0, mid, 0
	for i < mid && j < len(s) {
		if s[i] < s[j] {
			tmp[k] = s[i]
			i++
		} else {
			tmp[k] = s[j]
			j++
		}
		k++
	}
	for i < mid {
		tmp[k] = s[i]
		i++
		k++
	}
	for j < len(s) {
		tmp[k] = s[j]
		j++
		k++
	}
	copy(s, tmp)
}

func QuickSort(s []int) {
	if len(s) < 2 {
		return
	}
	left, right := 0, len(s)-1
	pivot := len(s) / 2
	s[pivot], s[right] = s[right], s[pivot]
	for i := range s {
		if s[i] < s[right] {
			s[i], s[left] = s[left], s[i]
			left++
		}
	}
	s[left], s[right] = s[right], s[left]
	QuickSort(s[:left])
	QuickSort(s[left+1:])
}

func SelectionSort(s []int) {
	n := len(s)
	for i := 0; i < n-1; i++ {
		minIdx := i
		for j := i + 1; j < n; j++ {
			if s[j] < s[minIdx] {
				minIdx = j
			}
		}
		s[i], s[minIdx] = s[minIdx], s[i]
	}
}

func InsertionSort(s []int) {
	n := len(s)
	for i := 1; i < n; i++ {
		key := s[i]
		j := i - 1
		for j >= 0 && s[j] > key {
			s[j+1] = s[j]
			j--
		}
		s[j+1] = key
	}
}
