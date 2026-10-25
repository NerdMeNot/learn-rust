// Package wordstat counts the words in a piece of text.
package wordstat

import (
	"errors"
	"strings"

	"example.com/wordstat/internal/words"
)

// ErrEmpty is returned when the text has no words.
var ErrEmpty = errors.New("no words to count")

// Count returns how often each word appears.
func Count(text string) (map[string]int, error) {
	counts := map[string]int{}
	for _, w := range strings.Fields(text) {
		counts[words.Normalize(w)]++
	}
	if len(counts) == 0 {
		return nil, ErrEmpty
	}
	return counts, nil
}
