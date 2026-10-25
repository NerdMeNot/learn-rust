package main

import (
	"fmt"
	"os"
)

func readConfig(path string) (string, error) {
	b, err := os.ReadFile(path)
	if err != nil {
		return "", err // any error value fits the error interface
	}
	return string(b), nil
}

func main() {
	if _, err := readConfig("missing.conf"); err != nil {
		fmt.Println(err)
	}
}
