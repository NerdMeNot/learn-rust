package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
)

func main() {
	dir, err := os.MkdirTemp("", "hello")
	if err != nil {
		panic(err)
	}
	cmd := exec.Command("go", "mod", "init", "hello")
	cmd.Dir = dir
	if err := cmd.Run(); err != nil {
		panic(err)
	}
	text, err := os.ReadFile(filepath.Join(dir, "go.mod"))
	if err != nil {
		panic(err)
	}
	fmt.Print("go.mod:\n", string(text))
}
