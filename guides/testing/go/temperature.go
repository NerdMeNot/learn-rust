package temperature

import (
	"fmt"
	"strconv"
)

// ToFahrenheit converts Celsius to Fahrenheit.
func ToFahrenheit(c float64) float64 { return c*9/5 + 32 }

// ParseCelsius parses text like "21C" or "70F" into Celsius.
func ParseCelsius(text string) (float64, error) {
	if len(text) < 2 {
		return 0, fmt.Errorf("too short: %q", text)
	}
	num, unit := text[:len(text)-1], text[len(text)-1:]
	v, err := strconv.ParseFloat(num, 64)
	if err != nil {
		return 0, fmt.Errorf("not a number: %q", num)
	}
	switch unit {
	case "C":
		return v, nil
	case "F":
		return (v - 32) * 5 / 9, nil
	}
	return 0, fmt.Errorf("unknown unit: %q", unit)
}
