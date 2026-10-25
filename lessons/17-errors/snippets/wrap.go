package main

import (
	"errors"
	"fmt"
	"strconv"
)

var ErrMissing = errors.New("missing")

func threads(settings map[string]string) (int, error) {
	text, ok := settings["threads"]
	if !ok {
		return 0, fmt.Errorf("setting %q: %w", "threads", ErrMissing)
	}
	n, err := strconv.Atoi(text)
	if err != nil {
		return 0, fmt.Errorf("setting %q: %w", "threads", err) // wrap, keep the cause
	}
	return n, nil
}

func main() {
	for _, settings := range []map[string]string{{}, {"threads": "many"}, {"threads": "8"}} {
		n, err := threads(settings)
		var numErr *strconv.NumError
		switch {
		case errors.Is(err, ErrMissing): // is it this error, anywhere in the chain?
			fmt.Println("missing:", err)
		case errors.As(err, &numErr): // is there a *NumError in the chain? get it
			fmt.Println("bad number:", numErr.Num, "->", numErr.Err)
		case err != nil:
			fmt.Println("other:", err)
		default:
			fmt.Println("threads:", n)
		}
	}
}
