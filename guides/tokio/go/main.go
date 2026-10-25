package main

import (
	"context"
	"fmt"
	"time"
)

func fetch(ctx context.Context, name string, ms int) (string, error) {
	select {
	case <-time.After(time.Duration(ms) * time.Millisecond):
		return fmt.Sprintf("%s done after %dms", name, ms), nil
	case <-ctx.Done():
		return "", ctx.Err()
	}
}

func main() {
	ctx := context.Background()

	users := make(chan string, 1)
	orders := make(chan string, 1)
	go func() { r, _ := fetch(ctx, "users", 30); users <- r }()
	go func() { r, _ := fetch(ctx, "orders", 10); orders <- r }()
	fmt.Println(<-users)
	fmt.Println(<-orders)

	left := make(chan string, 1)
	right := make(chan string, 1)
	go func() { r, _ := fetch(ctx, "left", 20); left <- r }()
	go func() { r, _ := fetch(ctx, "right", 20); right <- r }()
	fmt.Println(<-left, "/", <-right)

	tctx, cancel := context.WithTimeout(ctx, 50*time.Millisecond)
	defer cancel()
	if r, err := fetch(tctx, "slow", 500); err != nil {
		fmt.Println("slow call timed out")
	} else {
		fmt.Println(r)
	}
}
