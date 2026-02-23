package main

import (
	"group2/sorts"
)

func main() {
	values := sorts.ReadInput()
	sorts.InsertionSort(values)
	sorts.PrintOutput(values)
}
