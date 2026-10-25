package main

import (
	"errors"
	"fmt"
	"os"
	"os/exec"
	"strings"
	"sync"
)

// The worker program: a goroutine panics while it holds the lock, and nothing calls recover.
func worker() {
	var mu sync.Mutex
	data := []int{1, 2, 3}
	go func() {
		mu.Lock()
		defer mu.Unlock()
		data = append(data, 4)
		panic("worker failed halfway")
	}()
	select {} // main waits here; the panic ends the whole program first
}

func main() {
	if os.Getenv("AS_WORKER") == "1" {
		worker()
		return
	}
	// Run the worker program as a child process and report how it ended.
	cmd := exec.Command(os.Args[0])
	cmd.Env = append(os.Environ(), "AS_WORKER=1")
	out, err := cmd.CombinedOutput()
	var exit *exec.ExitError
	if errors.As(err, &exit) {
		fmt.Println("child exit code:", exit.ExitCode())
	}
	fmt.Println("child's first line:", strings.SplitN(string(out), "\n", 2)[0])
}
