package main

import (
	"fmt"

	"example.com/wordstat/internal/words"
)

func main() {
	fmt.Println(words.Normalize("Hello!"))
}
