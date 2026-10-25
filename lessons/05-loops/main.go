package main

import "fmt"

func main() {
	for i := 1; i < 4; i++ {
		fmt.Println("for", i)
	}

	n := 3
	for n > 0 {
		n--
	}

	tries, found := 0, 0
	for {
		tries++
		if tries == 3 {
			found = tries * 10
			break
		}
	}
	fmt.Printf("n=%d found=%d\n", n, found)
}
