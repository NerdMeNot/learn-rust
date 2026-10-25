package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
)

func write(root, path, text string) {
	full := filepath.Join(root, path)
	os.MkdirAll(filepath.Dir(full), 0o755)
	os.WriteFile(full, []byte(text), 0o644)
}

// Runs `go <args>` in the project and prints what it said, or "ok".
func goDo(root, label string, args ...string) {
	fmt.Println(label)
	cmd := exec.Command("go", args...)
	cmd.Dir = root
	out, err := cmd.CombinedOutput()
	fmt.Print(string(out))
	if err == nil {
		fmt.Println("  ok")
	}
}

func build(root, label string) { goDo(root, label, "build", "./...") }

func main() {
	root, _ := os.MkdirTemp("", "gopkg")
	write(root, "go.mod", "module demo\n\ngo 1.22\n")
	// Two files in one folder: both belong to package shop, and nothing declares them.
	write(root, "shop/shop.go", "package shop\n\nfunc Total(q int) int { return q * rate() }\n")
	write(root, "shop/rate.go", "package shop\n\nfunc rate() int { return 10 }\n")
	write(root, "main.go", "package main\n\nimport \"demo/shop\"\n\nfunc main() { println(shop.Total(3)) }\n")
	build(root, "1. lowercase name used inside its own package, from another file")

	write(root, "main.go", "package main\n\nimport \"demo/shop\"\n\nfunc main() { println(shop.rate()) }\n")
	build(root, "2. the same lowercase name used from package main")

	// Two packages that import each other.
	write(root, "main.go", "package main\n\nimport \"demo/a\"\n\nfunc main() { a.A() }\n")
	write(root, "a/a.go", "package a\n\nimport \"demo/b\"\n\nfunc A() { b.B() }\n")
	write(root, "b/b.go", "package b\n\nimport \"demo/a\"\n\nfunc B() { a.A() }\n")
	build(root, "3. package a imports b, and b imports a")

	// An internal/ folder: only code under its parent folder may import it.
	root, _ = os.MkdirTemp("", "gopkg")
	write(root, "go.mod", "module demo\n\ngo 1.22\n")
	write(root, "a/internal/x/x.go", "package x\n\nfunc F() int { return 1 }\n")
	write(root, "a/a.go", "package a\n\nimport \"demo/a/internal/x\"\n\nfunc A() int { return x.F() }\n")
	build(root, "4. a package under a/ imports a/internal/x")
	write(root, "b/b.go", "package b\n\nimport \"demo/a/internal/x\"\n\nfunc B() int { return x.F() }\n")
	build(root, "5. a package outside a/ imports a/internal/x")

	// A test file in package a_test is a separate package: it sees only exported names.
	root, _ = os.MkdirTemp("", "gopkg")
	write(root, "go.mod", "module demo\n\ngo 1.22\n")
	write(root, "a/a.go", "package a\n\nfunc secret() int { return 2 }\n\nfunc Open() int { return secret() }\n")
	write(root, "a/in_test.go", "package a\n\nimport \"testing\"\n\nfunc TestIn(t *testing.T) { _ = secret() }\n")
	goDo(root, "6. a test file in package a calls the unexported secret", "vet", "./a")
	write(root, "a/out_test.go", "package a_test\n\nimport (\n\t\"testing\"\n\t\"demo/a\"\n)\n\nfunc TestOut(t *testing.T) { _ = a.secret() }\n")
	goDo(root, "7. a test file in package a_test calls it", "vet", "./a")
}
