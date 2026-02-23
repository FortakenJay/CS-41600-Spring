package selection

import (
	"../../sorts"
)

func main() {
	values := sorts.ReadInput()
	sorts.SelectionSort(values)
	sorts.PrintOutput(values)
}
