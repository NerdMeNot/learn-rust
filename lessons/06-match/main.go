package main

import "fmt"

func main() {
	day := 6
	var kind string
	switch {
	case day >= 1 && day <= 5:
		kind = "weekday"
	case day == 6 || day == 7:
		kind = "weekend"
	default:
		kind = "not a day"
	}
	fmt.Println(kind)
}
