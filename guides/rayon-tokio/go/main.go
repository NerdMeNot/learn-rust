package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"sync"
)

type reading struct {
	sensor string
	value  int
}

func parseLine(line string) (reading, error) {
	sensor, value, ok := strings.Cut(line, ",")
	if !ok {
		return reading{}, fmt.Errorf("%q: no comma", line)
	}
	n, err := strconv.Atoi(strings.TrimSpace(value))
	if err != nil {
		return reading{}, fmt.Errorf("%q: %w", line, err)
	}
	return reading{strings.TrimSpace(sensor), n}, nil
}

// One goroutine per line. Each writes to its own slot, so the order is kept.
func parseAll(lines []string) ([]reading, error) {
	out := make([]reading, len(lines))
	errs := make([]error, len(lines))
	var wg sync.WaitGroup
	for i, line := range lines {
		wg.Add(1)
		go func() {
			defer wg.Done()
			out[i], errs[i] = parseLine(line)
		}()
	}
	wg.Wait()
	for _, err := range errs {
		if err != nil {
			return nil, err
		}
	}
	return out, nil
}

func main() {
	requests := [][]string{
		{"a,1", "b,2", "c,3"},
		{"d,10", "e,20"},
		{"f,7", "g,oops"},
	}

	// three "requests" handled at once
	type result struct {
		readings []reading
		err      error
	}
	results := make([]result, len(requests))
	var wg sync.WaitGroup
	for i, lines := range requests {
		wg.Add(1)
		go func() {
			defer wg.Done()
			results[i].readings, results[i].err = parseAll(lines)
		}()
	}
	wg.Wait()
	for i, r := range results {
		if r.err != nil {
			fmt.Printf("request %d: error: %v\n", i, r.err)
			continue
		}
		total := 0
		names := []string{}
		for _, rd := range r.readings {
			total += rd.value
			names = append(names, rd.sensor)
		}
		fmt.Printf("request %d: %s total %d\n", i, strings.Join(names, ","), total)
	}

	// a blocking call is just a call: the goroutine waits, the others keep running
	data, err := os.ReadFile("go.mod")
	if err != nil {
		panic(err)
	}
	fmt.Printf("go.mod has %d lines\n", strings.Count(string(data), "\n"))
}
