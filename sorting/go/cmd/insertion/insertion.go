package insertion

import (
	"../../sorts"
)

func main() {
	values := sorts.ReadInput()
	sorts.InsertionSort(values)
	sorts.PrintOutput(values)
}
