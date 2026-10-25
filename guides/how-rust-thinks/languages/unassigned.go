package main

import "fmt"

func main() {
	ready := false
	var port uint16 // no "unassigned" state: it starts as 0
	if ready {
		port = 8080
	}
	fmt.Println("listening on", port)
}
