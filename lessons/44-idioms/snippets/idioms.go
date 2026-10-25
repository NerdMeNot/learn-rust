package main

import (
	"errors"
	"fmt"
	"time"
)

// 1. A defined type is distinct from its base type, but keeps the base type's operators.
type UserID int

// 2. Functional options: each option is a function that changes the value being built.
type Request struct {
	URL     string
	Method  string
	Retries int
	Timeout time.Duration
}

type Option func(*Request)

func WithMethod(m string) Option { return func(r *Request) { r.Method = m } }
func WithRetries(n int) Option   { return func(r *Request) { r.Retries = n } }

func NewRequest(url string, opts ...Option) (*Request, error) {
	if url == "" {
		return nil, errors.New("missing url")
	}
	r := &Request{URL: url, Method: "GET", Retries: 3, Timeout: 30 * time.Second}
	for _, opt := range opts {
		opt(r)
	}
	return r, nil
}

// 3. The zero value: every type has one, and nothing can forbid it.
type Port struct{ n uint16 }

func main() {
	fmt.Println("UserID(7) + 1 =", UserID(7)+1)

	r, _ := NewRequest("https://example.org", WithMethod("POST"), WithRetries(0))
	fmt.Println(r.Method, r.URL, "retries="+fmt.Sprint(r.Retries), "timeout="+r.Timeout.String())
	_, err := NewRequest("", WithRetries(5))
	fmt.Println("error:", err)

	var zero Port
	fmt.Println("zero Port:", zero.n)
}
