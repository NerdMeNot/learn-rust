package main

import (
	"cmp"
	"fmt"
	"os"
	"strconv"
)

func Max[T cmp.Ordered](a, b T) T {
	if b > a {
		return b
	}
	return a
}

func apply(f func(int) int, x int) int { return f(x) }

func find(m map[string]int, k string) (int, bool) {
	v, ok := m[k]
	return v, ok
}

func read(path string) (string, error) {
	b, err := os.ReadFile(path)
	return string(b), err
}

type Log struct{ lines []string }

func (l *Log) Add(line string) { l.lines = append(l.lines, line) }
func (l Log) Count() int       { return len(l.lines) }

type Describe interface{ Describe() string }
type Square struct{ side int }

func (s Square) Describe() string { return fmt.Sprint("square ", s.side) }

func show(d Describe) string { return d.Describe() }

func run() error { return nil }

// a generic type's methods repeat its type parameters in the receiver
type Store[K comparable, V any] struct{ m map[K]V }

func (s *Store[K, V]) Entry(key K) V { return s.m[key] }

func main() {
	if (&Store[string, int]{m: map[string]int{"a": 1}}).Entry("a") != 1 {
		panic("Store")
	}
	count := 0
	bump := func() { count++ } // a Go closure may change what it captured
	bump()
	bump()
	if count != 2 || show(Square{2}) != "square 2" || run() != nil {
		panic("closures and interfaces")
	}
	if Max(1.5, 0.5) != 1.5 || Max("a", "b") != "b" {
		panic("Max")
	}
	if apply(func(x int) int { return x + 1 }, 1) != 2 {
		panic("apply")
	}
	if _, ok := find(map[string]int{"a": 1}, "b"); ok {
		panic("find")
	}
	if _, err := read("/nope"); err == nil {
		panic("read")
	}
	l := Log{}
	l.Add("hello")
	if l.Count() != 1 {
		panic("Log")
	}
	fmt.Println("int bits:", strconv.IntSize)
	fmt.Println("go rows ok")
}
