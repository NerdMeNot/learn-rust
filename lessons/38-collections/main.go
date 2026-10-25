package main

import (
	"container/heap"
	"fmt"
	"maps"
	"slices"
	"sort"
	"strings"
)

// container/heap needs you to write these five methods
type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *IntHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *IntHeap) Pop() any {
	old := *h
	x := old[len(old)-1]
	*h = old[:len(old)-1]
	return x
}

func main() {
	text := "the cat saw the dog and the cat ran"

	counts := map[string]int{}
	for _, w := range strings.Fields(text) {
		counts[w]++
	}
	type pair struct {
		word string
		n    int
	}
	var pairs []pair
	for w, n := range counts {
		pairs = append(pairs, pair{w, n})
	}
	sort.Slice(pairs, func(i, j int) bool {
		if pairs[i].n != pairs[j].n {
			return pairs[i].n > pairs[j].n
		}
		return pairs[i].word < pairs[j].word
	})
	fmt.Println("top:", pairs[:3])

	// no sorted map: sort the keys yourself
	byName := map[string]int{"zoe": 31, "ana": 30, "mia": 28}
	fmt.Print("sorted:")
	for _, k := range slices.Sorted(maps.Keys(byName)) {
		fmt.Print(" ", k, "=", byName[k])
	}
	fmt.Println()

	// a set is a map to empty structs
	seen := map[string]struct{}{}
	for _, w := range strings.Fields(text) {
		seen[w] = struct{}{}
	}
	_, hasDog := seen["dog"]
	fmt.Println(len(seen), "unique words, has dog:", hasDog)

	queue := []string{"a", "b"}
	queue = append(queue, "c")
	first := queue[0]
	queue = queue[1:]
	fmt.Println("served", first, "waiting", queue)

	h := &IntHeap{}
	for _, n := range []int{5, 1, 8, 3} {
		heap.Push(h, n)
	}
	fmt.Print("smallest first:")
	for h.Len() > 0 {
		fmt.Print(" ", heap.Pop(h))
	}
	fmt.Println()
}
