package main

import "fmt"

func main() {
	var r float64
	fmt.Scan(&r)

	var pi float64 = 3.14159
	area := pi * (r * r)

	fmt.Printf("A=%.4f\n", area)
}
