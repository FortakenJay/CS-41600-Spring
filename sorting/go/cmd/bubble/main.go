package main

import (
	"group2/sorts"
)

func main() {
	values := sorts.ReadInput()
	sorts.BubbleSort(values)
	sorts.PrintOutput(values)
}
