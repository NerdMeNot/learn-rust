package main

import (
	"encoding/json"
	"fmt"
)

type Person struct {
	FullName string   `json:"fullName"`
	Age      int      `json:"age"`
	Email    *string  `json:"email"` // pointer, so it can be nil
	Tags     []string `json:"tags"`
}

func main() {
	text := `{ "fullName": "Ana Silva", "age": 31, "tags": ["admin"] }`
	var ana Person
	if err := json.Unmarshal([]byte(text), &ana); err != nil {
		panic(err)
	}
	fmt.Printf("%+v\n", ana)

	email := "bo@example.com"
	bo := Person{FullName: "Bo Chen", Age: 28, Email: &email, Tags: []string{}}
	out, _ := json.MarshalIndent(bo, "", "  ")
	fmt.Println(string(out))
}
