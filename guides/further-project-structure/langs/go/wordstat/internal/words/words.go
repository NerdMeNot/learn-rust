// Package words is internal: only code under example.com/wordstat may import it.
package words

import "strings"

func Normalize(w string) string {
	return strings.ToLower(strings.Trim(w, ".,!?;:\"'()"))
}
