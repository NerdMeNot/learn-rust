package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
)

// The same program, built twice: once in a module whose go.mod says
// `go 1.21`, once in a module that says `go 1.22`.
const program = `package main

import "fmt"

func main() {
	var fs []func() int
	for i := 0; i < 3; i++ {
		fs = append(fs, func() int { return i })
	}
	for _, f := range fs {
		fmt.Print(f(), " ")
	}
	fmt.Println()

	var gs []func() int
	for _, v := range []int{0, 1, 2} {
		gs = append(gs, func() int { return v })
	}
	for _, g := range gs {
		fmt.Print(g(), " ")
	}
	fmt.Println()
}
`

func run(goLine string) string {
	dir, _ := os.MkdirTemp("", "loopvar")
	defer os.RemoveAll(dir)
	os.WriteFile(filepath.Join(dir, "go.mod"), []byte("module demo\n\ngo "+goLine+"\n"), 0o644)
	os.WriteFile(filepath.Join(dir, "main.go"), []byte(program), 0o644)
	cmd := exec.Command("go", "run", ".")
	cmd.Dir = dir
	out, err := cmd.CombinedOutput()
	if err != nil {
		return fmt.Sprint(err, string(out))
	}
	return string(out)
}

func main() {
	// each run prints the three-clause `for` loop's values, then the `range` loop's
	fmt.Println("go.mod says go 1.21:")
	fmt.Print(run("1.21"))
	fmt.Println("go.mod says go 1.22:")
	fmt.Print(run("1.22"))
}
