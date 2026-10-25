package main

import (
	"fmt"
	"os"
)

func main() {
	port := os.Getenv("RBC_PORT")
	fmt.Println("listening on port", port)
}
