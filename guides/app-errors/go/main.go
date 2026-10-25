package main

import (
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type BadPortError struct{ Value string }

func (e *BadPortError) Error() string { return fmt.Sprintf("port must be a number, got %q", e.Value) }

func readConfig(path string) (string, error) {
	b, err := os.ReadFile(path)
	if err != nil {
		return "", fmt.Errorf("config file not found: %s", path)
	}
	return string(b), nil
}

func parsePort(text string) (int, error) {
	text = strings.TrimSpace(text)
	n, err := strconv.Atoi(text)
	if err != nil {
		return 0, &BadPortError{Value: text}
	}
	return n, nil
}

func loadPort(path string) (int, error) {
	text, err := readConfig(path)
	if err != nil {
		return 0, fmt.Errorf("loading settings: %w", err)
	}
	port, err := parsePort(text)
	if err != nil {
		return 0, fmt.Errorf("reading %s: %w", path, err)
	}
	return port, nil
}

func main() {
	os.WriteFile("good.conf", []byte("8080\n"), 0o644)
	os.WriteFile("bad.conf", []byte("eighty\n"), 0o644)

	for _, path := range []string{"good.conf", "bad.conf", "missing.conf"} {
		if port, err := loadPort(path); err != nil {
			fmt.Printf("%s: %v\n", path, err)
		} else {
			fmt.Printf("%s: port %d\n", path, port)
		}
	}

	_, err := loadPort("bad.conf")
	var bad *BadPortError
	if errors.As(err, &bad) {
		fmt.Printf("the bad value was %q\n", bad.Value)
	}
}
