package main

import "fmt"

type Config struct {
	Name    string
	Port    uint16
	Verbose bool
}

func main() {
	base := Config{Name: "service", Port: 8080, Verbose: false}
	test := base // a struct assignment copies the fields
	test.Port = 9000
	fmt.Printf("%+v\n", base)
	fmt.Printf("%+v\n", test)
}
