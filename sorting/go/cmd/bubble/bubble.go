package bubble

import (
	"../../sorts"
)

func main() {
	values := sorts.ReadInput()
	sorts.BubbleSort(values)
	sorts.PrintOutput(values)
}
