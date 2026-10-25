package main

import "fmt"

func main() {
	for i := 0; i < 2; i++ {
		defer fmt.Println("deferred", i)
		fmt.Println("end of iteration", i)
	}
	fmt.Println("end of function")
}
