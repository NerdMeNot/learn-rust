package main

import "fmt"

func logMsg(msg string) {
	fmt.Println("log: " + msg)
}

func main() {
	r := logMsg("hello")
	fmt.Println(r)
}
