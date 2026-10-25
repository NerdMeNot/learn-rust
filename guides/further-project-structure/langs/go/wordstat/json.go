//go:build json

package wordstat

import "encoding/json"

// ToJSON is only compiled with `-tags json`.
func ToJSON(counts map[string]int) string {
	b, _ := json.Marshal(counts)
	return string(b)
}
