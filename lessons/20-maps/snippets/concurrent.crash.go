package main

import "sync"

func main() {
	m := map[int]int{}
	var wg sync.WaitGroup
	for g := 0; g < 4; g++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for i := 0; i < 100000; i++ {
				m[i] = i
			}
		}()
	}
	wg.Wait()
}
