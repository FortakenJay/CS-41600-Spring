package main

import (
	"group2/sorts"
)

func main() {
	values := sorts.ReadInput()
	sorts.QuickSort(values)
	sorts.PrintOutput(values)
}
