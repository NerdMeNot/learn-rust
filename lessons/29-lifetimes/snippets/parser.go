package main

import (
	"fmt"
	"strings"
)

type Parser struct {
	rest string
}

func (p *Parser) NextWord() (string, bool) {
	rest := strings.TrimLeft(p.rest, " ")
	if rest == "" {
		return "", false
	}
	word, tail, _ := strings.Cut(rest, " ")
	p.rest = tail
	return word, true
}

func main() {
	parser := &Parser{rest: "lifetimes are regions"}
	a, _ := parser.NextWord()
	b, _ := parser.NextWord() // a is still usable: no annotation needed
	fmt.Println("a:", a)
	fmt.Println("b:", b)
}
