package main

import (
	"group2/sorts"
)

func main() {
	values := sorts.ReadInput()
	sorts.SelectionSort(values)
	sorts.PrintOutput(values)
}
