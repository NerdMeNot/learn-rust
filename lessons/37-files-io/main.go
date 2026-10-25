package main

import (
	"bufio"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
)

func main() {
	path := filepath.Join(os.TempDir(), "rbc-notes.txt")

	if err := os.WriteFile(path, []byte("milk\neggs\n"), 0o644); err != nil {
		panic(err)
	}

	f, err := os.OpenFile(path, os.O_APPEND|os.O_WRONLY, 0)
	if err != nil {
		panic(err)
	}
	fmt.Fprintln(f, "bread")
	f.Close()

	all, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	fmt.Println(len(all), "bytes")

	r, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	sc := bufio.NewScanner(r)
	for i := 1; sc.Scan(); i++ {
		fmt.Printf("%d: %s\n", i, sc.Text())
	}
	r.Close()

	fmt.Printf("file name: %q\n", filepath.Base(path))

	_, err = os.ReadFile("/no/such/file.txt")
	fmt.Println("error:", err, "| not found:", errors.Is(err, fs.ErrNotExist))

	os.Remove(path)
}
