package main

import (
	"fmt"
	"reflect"
)

type User struct {
	Name string `json:"name"`
}

func main() {
	field, _ := reflect.TypeOf(User{}).FieldByName("Name")
	fmt.Println("tag read at run time:", field.Tag.Get("json"))
}
