package main

import (
	"group2/sorts"
)

func main() {
	values := sorts.ReadInput()
	sorts.MergeSort(values)
	sorts.PrintOutput(values)
}
