package main

import "fmt"

// the caller's slice goes in, and the grown slice comes back
func appendGreeting(dst []byte, name string) []byte {
	return fmt.Appendf(dst, "hello %s", name)
}

func main() {
	buf := make([]byte, 0, 64)
	for _, name := range []string{"ana", "bo", "cy"} {
		buf = appendGreeting(buf[:0], name)
		fmt.Println(string(buf))
	}
	fmt.Println("capacity:", cap(buf))
}
