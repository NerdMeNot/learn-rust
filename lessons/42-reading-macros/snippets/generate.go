package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

// A harness: it writes a three-file module to a temporary folder and runs go commands in it.
// gen.go is the generator that `go generate` runs.
var files = map[string]string{
	"go.mod": "module demo\n\ngo 1.27\n",
	"gen.go": `//go:build ignore

package main

import "os"

func main() {
	src := "package main\n\nfunc colors() []string { return []string{\"red\", \"green\"} }\n"
	os.WriteFile("colors_gen.go", []byte(src), 0o644)
}
`,
	"main.go": `package main

//go:generate go run gen.go

import "fmt"

func main() { fmt.Println(colors()) }
`,
}

func run(dir string, args ...string) {
	cmd := exec.Command(args[0], args[1:]...)
	cmd.Dir = dir
	out, err := cmd.CombinedOutput()
	fmt.Printf("$ %s\n%s", strings.Join(args, " "), out)
	if err != nil {
		fmt.Println("error:", err)
	}
}

func main() {
	dir, _ := os.MkdirTemp("", "gogen")
	defer os.RemoveAll(dir)
	for name, text := range files {
		os.WriteFile(filepath.Join(dir, name), []byte(text), 0o644)
	}
	run(dir, "go", "run", ".")
	run(dir, "go", "generate")
	run(dir, "cat", "colors_gen.go")
	run(dir, "go", "run", ".")
}
