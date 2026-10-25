package main

import (
	"errors"
	"fmt"
	"io"
	"os"
	"sort"
	"strconv"
	"strings"
	"unicode"
	"unicode/utf8"
)

const defaultTop = 10
const usage = "usage: wordcount [FILE] [--top N]"

func words(text string) []string {
	pieces := strings.FieldsFunc(text, func(r rune) bool {
		return !unicode.IsLetter(r) && r != '\''
	})
	var found []string
	for _, piece := range pieces {
		if word := strings.Trim(piece, "'"); word != "" {
			found = append(found, word)
		}
	}
	return found
}

type entry struct {
	word  string
	count int
}

func countWords(text string) map[string]int {
	counts := map[string]int{}
	for _, word := range words(text) {
		counts[strings.ToLower(word)]++
	}
	return counts
}

func topN(counts map[string]int, n int) []entry {
	entries := make([]entry, 0, len(counts))
	for word, count := range counts {
		entries = append(entries, entry{word, count})
	}
	sort.Slice(entries, func(i, j int) bool {
		if entries[i].count != entries[j].count {
			return entries[i].count > entries[j].count
		}
		return entries[i].word < entries[j].word
	})
	if len(entries) > n {
		entries = entries[:n]
	}
	return entries
}

func parseArgs(args []string) (path string, top int, err error) {
	top = defaultTop
	for i := 0; i < len(args); i++ {
		arg := args[i]
		switch {
		case arg == "--top":
			i++
			if i == len(args) {
				return "", 0, errors.New("--top needs a number")
			}
			n, convErr := strconv.Atoi(args[i])
			if convErr != nil {
				return "", 0, fmt.Errorf("invalid value %q for --top", args[i])
			}
			if n < 1 {
				return "", 0, errors.New("--top must be at least 1")
			}
			top = n
		case strings.HasPrefix(arg, "-") && arg != "-":
			return "", 0, errors.New("unknown option " + arg)
		case path != "":
			return "", 0, errors.New("only one input file is allowed")
		case arg != "-":
			path = arg
		}
	}
	return path, top, nil
}

func readInput(path string) (string, error) {
	var data []byte
	var err error
	if path == "" {
		data, err = io.ReadAll(os.Stdin)
	} else {
		data, err = os.ReadFile(path)
	}
	return string(data), err
}

func run() int {
	path, top, err := parseArgs(os.Args[1:])
	if err != nil {
		fmt.Fprintf(os.Stderr, "wordcount: %v\n%s\n", err, usage)
		return 2
	}
	text, err := readInput(path)
	if err != nil {
		what := path
		if what == "" {
			what = "standard input"
		}
		fmt.Fprintf(os.Stderr, "wordcount: cannot read %s: %v\n", what, err)
		return 1
	}

	counts := countWords(text)
	rows := topN(counts, top)
	total := 0
	for _, n := range counts {
		total += n
	}
	fmt.Printf("%d words, %d distinct\n", total, len(counts))
	width := 0
	for _, row := range rows {
		width = max(width, utf8.RuneCountInString(row.word))
	}
	for i, row := range rows {
		fmt.Printf("%3d  %-*s  %5d\n", i+1, width, row.word, row.count)
	}
	return 0
}

func main() { os.Exit(run()) }
