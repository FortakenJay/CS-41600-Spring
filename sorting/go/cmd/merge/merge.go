package merge

import (
	"../../sorts"
)

func main() {
	values := sorts.ReadInput()
	sorts.MergeSort(values)
	sorts.PrintOutput(values)
}
