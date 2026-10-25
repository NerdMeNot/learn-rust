package main

import (
	"fmt"
	"os"
	"path/filepath"
)

func main() {
	path := filepath.Join(os.TempDir(), "rbc-hello.txt")
	os.WriteFile(path, []byte("hello"), 0o644)

	f, _ := os.Open(path)
	buf := make([]byte, 64)
	n, _ := f.Read(buf)
	fmt.Println(string(buf[:n]))
	f.Close()
	os.Remove(path)
}
