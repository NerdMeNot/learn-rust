package main

import (
	"fmt"
	"os"
	"os/exec"
	"strings"
)

func depth(n int) int {
	if n == 0 {
		return 0
	}
	return 1 + depth(n-1)
}

func main() {
	if len(os.Args) > 1 { // the child process: the overflow happens here
		defer func() {
			recover() // does not help: a stack overflow is a fatal error, not a panic
		}()
		fmt.Println("depth(100_000_000):", depth(100_000_000))
		return
	}
	fmt.Println("depth(500):", depth(500))
	// Run this program again as a child, and keep only its "fatal error" line,
	// because Go also prints hundreds of lines of trace.
	out, err := exec.Command(os.Args[0], "child").CombinedOutput()
	for _, line := range strings.Split(string(out), "\n") {
		if strings.HasPrefix(line, "fatal error") {
			fmt.Println(line)
		}
	}
	fmt.Println("child:", err)
}
