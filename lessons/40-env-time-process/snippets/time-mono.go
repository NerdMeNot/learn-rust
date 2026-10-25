package main

import (
	"fmt"
	"strings"
	"time"
)

func main() {
	// time.Now() carries a wall-clock reading and a monotonic one ("m=+")
	now := time.Now()
	fmt.Println("time.Now() has a monotonic reading:", strings.Contains(now.String(), "m=+"))
	fmt.Println("Round(0) strips it:", !strings.Contains(now.Round(0).String(), "m=+"))
	// a time.Time that did not come from time.Now() has none
	fmt.Println("time.Unix(0, 0) has one:", strings.Contains(time.Unix(0, 0).String(), "m=+"))

	// time.Duration is a signed count of nanoseconds
	fmt.Println("1s - 2s =", time.Second-2*time.Second)
}
