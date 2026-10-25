package main

import (
	"encoding/json"
	"fmt"
)

type Person struct {
	FullName string `json:"fullName"`
	Age      int    `json:"age"`
}

func main() {
	var cy Person
	err := json.Unmarshal([]byte(`{ "fullName": "Cy" }`), &cy) // no "age"
	fmt.Printf("%+v err=%v\n", cy, err)
}
