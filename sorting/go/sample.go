package main

import "fmt"

func main() {
    fmt.Println("Hello world")
    m, n := named_return(5)
    fmt.Printf("m=%d,  n=%d\n", m, n)
    m2, n2 := named_return2(5)
    fmt.Printf("m2=%d, n2=%d\n", m2, n2)
}

func named_return(a int) (x int, y int) {
    x = a+1
    y = a*2
    return y, x
}

func named_return2(a int) (x int, y int) {
    x = a+1
    y = a*2
    return
}
