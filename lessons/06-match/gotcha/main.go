package main

import "fmt"

func main() {
	day := 6
	kind := ""
	switch {
	case day >= 1 && day <= 5:
		kind = "weekday"
	case day == 6 || day == 7:
		kind = "weekend"
	}
	fmt.Println(kind)
}
