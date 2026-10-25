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

type Item struct{ name string }

func main() {
	items := []Item{{"a"}, {"b"}, {"c"}}
	var ptrs []*Item
	for _, it := range items {
		ptrs = append(ptrs, &it)
	}
	for _, p := range ptrs {
		fmt.Print(p.name, " ")
	}
	fmt.Println()
}
`

func run(goLine string) string {
	dir, _ := os.MkdirTemp("", "loopptr")
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
	fmt.Println("go.mod says go 1.21:")
	fmt.Print(run("1.21"))
	fmt.Println("go.mod says go 1.22:")
	fmt.Print(run("1.22"))
}
