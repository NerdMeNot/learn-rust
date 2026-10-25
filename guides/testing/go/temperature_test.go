package temperature

import "testing"

func TestBoilingPoint(t *testing.T) {
	if got := ToFahrenheit(100); got != 212 {
		t.Errorf("got %v, want 212", got)
	}
}

func TestParsesFahrenheit(t *testing.T) {
	got, err := ParseCelsius("212F")
	if err != nil || got != 100 {
		t.Errorf("got %v, %v; want 100, nil", got, err)
	}
}

func TestRejectsUnknownUnit(t *testing.T) {
	if _, err := ParseCelsius("20K"); err == nil {
		t.Error("expected an error")
	}
}
