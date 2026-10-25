package main

import (
	"bytes"
	"fmt"
	"os"
	"os/exec"
)

func main() {
	if len(os.Args) > 1 {
		fmt.Println("x =", 42)
		println("from println")
		return
	}
	var stdout, stderr bytes.Buffer
	cmd := exec.Command(os.Args[0], "child")
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	if err := cmd.Run(); err != nil {
		panic(err)
	}
	fmt.Printf("stdout: %q\nstderr: %q\n", stdout.String(), stderr.String())
}
