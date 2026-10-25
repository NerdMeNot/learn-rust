package main

import "fmt"

type Config struct{ Name string }

func main() {
	var p *Config // nil: it points at nothing
	fmt.Println("about to read p.Name")
	fmt.Println(p.Name)
}
