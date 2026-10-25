package main

import (
	"encoding/json"
	"fmt"
)

type Item struct {
	Name string `json:"name"`
	Qty  int    `json:"qty"`
}

func main() {
	out, err := json.Marshal(Item{"apple", 3})
	fmt.Println(string(out), err)
}
