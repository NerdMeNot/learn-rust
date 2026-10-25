package main

import (
	"bufio"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"unicode/utf8"
)

// how many write system calls this process has made so far (Linux: the syscw line)
func writeCalls() int {
	b, _ := os.ReadFile("/proc/self/io")
	for _, l := range strings.Split(string(b), "\n") {
		if v, ok := strings.CutPrefix(l, "syscw:"); ok {
			n, _ := strconv.Atoi(strings.TrimSpace(v))
			return n
		}
	}
	return -1
}

func main() {
	_, err := os.ReadFile("/no/such/file.txt")
	fmt.Println("error:", err)
	fmt.Println("errors.Is(err, fs.ErrNotExist):", errors.Is(err, fs.ErrNotExist))
	var pe *fs.PathError
	if errors.As(err, &pe) {
		fmt.Println("the path is in the error:", pe.Path)
	}

	fmt.Println("filepath.Join(absolute):", filepath.Join("/data", "/etc/hosts"))

	// invalid UTF-8 is not an error: ReadFile gives bytes and string() accepts anything
	path := filepath.Join(os.TempDir(), "lr-fio-go.txt")
	os.WriteFile(path, []byte("bad \xff here\n"), 0o644)
	b, err := os.ReadFile(path)
	fmt.Println("read:", len(b), "bytes, err:", err, ", utf8.Valid:", utf8.Valid(b))

	// bufio.Scanner caps a line at 64 KiB unless you call Buffer to raise the limit
	data := strings.Repeat("a", 70000) + "\nshort\n"
	sc := bufio.NewScanner(strings.NewReader(data))
	lines := 0
	for sc.Scan() {
		lines++
	}
	fmt.Println("scanner lines:", lines, "err:", sc.Err())

	// bufio.Reader.ReadString has no cap: like Rust's lines(), it grows to fit the line
	br := bufio.NewReader(strings.NewReader(data))
	for {
		line, err := br.ReadString('\n')
		fmt.Println("ReadString:", len(line), err)
		if err != nil {
			break
		}
	}

	// an os.File is unbuffered, like a Rust File: every Fprintf is a write system call
	f, _ := os.Create(path)
	before := writeCalls()
	for i := 0; i < 1000; i++ {
		fmt.Fprintf(f, "line %d\n", i)
	}
	direct := writeCalls() - before
	w := bufio.NewWriter(f)
	before = writeCalls()
	for i := 0; i < 1000; i++ {
		fmt.Fprintf(w, "line %d\n", i)
	}
	w.Flush()
	fmt.Println("os.File:", direct, "write calls; bufio.Writer:", writeCalls()-before)
	f.Close()
	os.Remove(path)
}
