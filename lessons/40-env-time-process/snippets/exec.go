package main

import (
	"errors"
	"fmt"
	"os"
	"os/exec"
	"strings"
)

func main() {
	// arguments are passed as they are, with no shell
	out, err := exec.Command("echo", "a  b", "$HOME", "one; echo two").Output()
	fmt.Printf("output %q, err %v\n", out, err)

	// a program that runs and fails comes back as an error (Rust: Ok(status))
	err = exec.Command("false").Run()
	var exit *exec.ExitError
	fmt.Printf("false: %v, is ExitError: %v\n", err, errors.As(err, &exit))

	// a program that cannot start
	err = exec.Command("echo hello").Run()
	fmt.Printf("%q: %v\n", "echo hello", err)

	// Run() sends the child's output nowhere (Stdout is nil); Rust's status() lets it print
	fmt.Println("Run() on echo:")
	exec.Command("echo", "this line is discarded").Run()
	fmt.Println("(nothing above)")

	// Stdin takes a reader: Go writes it and closes the pipe for you
	sorter := exec.Command("sort")
	sorter.Stdin = strings.NewReader("pear\napple\n")
	sorted, _ := sorter.Output()
	fmt.Printf("sort printed %q\n", sorted)

	// os.Args includes the program name; Setenv is an ordinary function
	fmt.Println("args after the program:", len(os.Args)-1)
	os.Setenv("RBC_MODE", "fast")
	fmt.Println("RBC_MODE =", os.Getenv("RBC_MODE"))
}
