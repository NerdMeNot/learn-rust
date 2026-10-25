package main

import (
	"context"
	"errors"
	"fmt"
	"os/exec"
	"time"
)

func main() {
	// a deadline for a child: when the context expires, Go kills it
	ctx, cancel := context.WithTimeout(context.Background(), 200*time.Millisecond)
	defer cancel()
	err := exec.CommandContext(ctx, "sleep", "60").Run()
	fmt.Println("deadline exceeded:", errors.Is(ctx.Err(), context.DeadlineExceeded), "-", err)

	// a child ended by a signal has ExitCode() -1, and the text names the signal
	err = exec.Command("sh", "-c", "kill -9 $$").Run()
	var exit *exec.ExitError
	if errors.As(err, &exit) {
		fmt.Printf("kill -9: ExitCode() %d, %v\n", exit.ExitCode(), exit)
	}
}
