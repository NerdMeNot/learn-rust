package parse

import "testing"

func TestSeconds(t *testing.T) {
	cases := []struct {
		name    string
		input   string
		want    uint64
		wantErr bool
	}{
		{"seconds", "90s", 90, false},
		{"minutes", "2m", 120, false},
		{"hours and minutes", "1h30m", 5400, false},
		{"empty", "", 0, true},
		{"no unit", "15", 0, true},
		{"unknown unit", "5x", 0, true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got, err := Seconds(tc.input)
			if (err != nil) != tc.wantErr || got != tc.want {
				t.Errorf("Seconds(%q) = %d, %v; want %d, error %v", tc.input, got, err, tc.want, tc.wantErr)
			}
		})
	}
}
