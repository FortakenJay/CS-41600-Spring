package quick

import (
	"../../sorts"
)

func main() {
	values := sorts.ReadInput()
	sorts.QuickSort(values)
	sorts.PrintOutput(values)
}
