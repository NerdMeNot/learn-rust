package main

import "fmt"

func main() {
	w := make([]int, 4, 8) // len 4, room for 8
	copy(w, []int{1, 2, 3, 4})

	part := w[1:3]
	part = append(part, 99) // there is spare room, so this writes into w
	fmt.Println("w:", w, "part:", part)

	fmt.Println("w[1:6]:", w[1:6]) // reslicing past len, up to cap, is allowed
}
