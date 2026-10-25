package main

import (
	"fmt"
	"strconv"
)

// Port is valid by construction, if every caller goes through ParsePort.
// In package main the field is visible to the whole package; in its own
// package, `n` would be hidden from other packages.
type Port struct{ n uint16 }

func ParsePort(text string) (Port, error) {
	n, err := strconv.ParseUint(text, 10, 32)
	if err != nil {
		return Port{}, fmt.Errorf("not a number")
	}
	if n == 0 || n > 65535 {
		return Port{}, fmt.Errorf("%d is outside 1-65535", n)
	}
	return Port{uint16(n)}, nil
}

func listen(p Port) {
	fmt.Println("listening on", p.n)
}

func main() {
	for _, text := range []string{"8080", "0", "http"} {
		port, err := ParsePort(text)
		if err != nil {
			fmt.Printf("%q: %v\n", text, err)
			continue
		}
		listen(port)
	}
}
