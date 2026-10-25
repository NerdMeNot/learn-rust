package main

import (
	"fmt"
	"runtime"
	"sort"
	"sync"
)

// chunks splits 0..n into one range per CPU and runs fn on each range in its own goroutine.
func chunks(n int, fn func(part, lo, hi int)) {
	parts := runtime.NumCPU()
	var wg sync.WaitGroup
	for p := 0; p < parts; p++ {
		lo, hi := n*p/parts, n*(p+1)/parts
		wg.Add(1)
		go func() {
			defer wg.Done()
			fn(p, lo, hi)
		}()
	}
	wg.Wait()
}

func main() {
	numbers := make([]uint64, 1_000_000)
	for i := range numbers {
		numbers[i] = uint64(i + 1)
	}

	// Sum of squares: each goroutine fills its own slot, then we add the slots.
	partial := make([]uint64, runtime.NumCPU())
	chunks(len(numbers), func(part, lo, hi int) {
		for _, n := range numbers[lo:hi] {
			partial[part] += n * n
		}
	})
	var total uint64
	for _, s := range partial {
		total += s
	}
	fmt.Println("sum of squares:", total)

	// Squares in input order: each goroutine writes only its own indexes.
	squares := make([]uint64, len(numbers))
	chunks(len(numbers), func(_, lo, hi int) {
		for i := lo; i < hi; i++ {
			squares[i] = numbers[i] * numbers[i]
		}
	})
	inOrder := true
	for i, s := range squares {
		inOrder = inOrder && s == uint64(i+1)*uint64(i+1)
	}
	fmt.Println("collected squares in input order:", inOrder)

	// Multiples of 7: one counter per chunk again.
	counts := make([]int, runtime.NumCPU())
	chunks(len(numbers), func(part, lo, hi int) {
		for _, n := range numbers[lo:hi] {
			if n%7 == 0 {
				counts[part]++
			}
		}
	})
	sevens := 0
	for _, c := range counts {
		sevens += c
	}
	fmt.Println("multiples of 7:", sevens)

	// In place.
	data := []int{3, 1, 2}
	chunks(len(data), func(_, lo, hi int) {
		for i := lo; i < hi; i++ {
			data[i] *= 10
		}
	})
	fmt.Println("times ten, in place:", data)

	// sort.Ints is an ordinary sequential sort.
	sort.Ints(data)
	fmt.Println("sorted:", data)
}
