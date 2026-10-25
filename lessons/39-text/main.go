package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	line := "  name=Ana, age=30, city=Lisbon  "

	clean := strings.TrimSpace(line)
	fmt.Printf("[%s]\n", clean)

	for _, field := range strings.Split(clean, ",") {
		if key, value, ok := strings.Cut(strings.TrimSpace(field), "="); ok {
			fmt.Printf("%5s -> %s\n", key, value)
		}
	}

	age, err := strconv.Atoi("30")
	if err != nil {
		age = 0
	}
	fmt.Println("next year:", age+1)

	shout := strings.ReplaceAll(strings.ToUpper(clean), ", ", " | ")
	fmt.Println(shout)

	fmt.Println("starts with name:", strings.HasPrefix(clean, "name"))
	fmt.Println("position of age:", strings.Index(clean, "age"))

	fmt.Println(strings.Join([]string{"build", "with", "rust"}, "-"))

	var b strings.Builder
	for i := 1; i <= 3; i++ {
		b.WriteString(strconv.Itoa(i))
		b.WriteByte(';')
	}
	fmt.Println(b.String())
}
