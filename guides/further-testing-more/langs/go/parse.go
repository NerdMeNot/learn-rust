package parse

import (
	"errors"
	"fmt"
	"strconv"
)

// Seconds reads text like "1h30m" or "90s" and returns the number of seconds.
func Seconds(text string) (uint64, error) {
	if text == "" {
		return 0, errors.New("empty")
	}
	var total uint64
	digits := ""
	for _, c := range text {
		if c >= '0' && c <= '9' {
			digits += string(c)
			continue
		}
		per := map[rune]uint64{'s': 1, 'm': 60, 'h': 3600}[c]
		if per == 0 {
			return 0, fmt.Errorf("unknown unit %q", c)
		}
		if digits == "" {
			return 0, errors.New("no number")
		}
		n, err := strconv.ParseUint(digits, 10, 64)
		if err != nil {
			return 0, errors.New("too big")
		}
		total += n * per
		digits = ""
	}
	if digits != "" {
		return 0, errors.New("missing unit")
	}
	return total, nil
}
