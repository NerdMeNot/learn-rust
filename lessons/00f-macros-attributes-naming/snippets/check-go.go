package main

import "fmt"

var bad = false

func same(got, want string) {
	if got != want {
		fmt.Printf("got [%s], want [%s]\n", got, want)
		bad = true
	}
}

func main() {
	same(fmt.Sprintf("%5d", 42), "   42")
	same(fmt.Sprintf("%-5d", 42), "42   ")
	same(fmt.Sprintf("%05d", 42), "00042")
	same(fmt.Sprintf("%+d", 42), "+42")
	same(fmt.Sprintf("%.2f", 3.14159), "3.14")
	same(fmt.Sprintf("%8.3f", 3.14159), "   3.142")
	same(fmt.Sprintf("%x", 255), "ff")
	same(fmt.Sprintf("%#x", 255), "0xff")
	same(fmt.Sprintf("%10s", "Ada"), "       Ada")
	same(fmt.Sprintf("%-10s", "Ada"), "Ada       ")
	same(fmt.Sprintf("%.3s", "Lovelace"), "Lov")
	same(fmt.Sprintf("%d %s", 42, "Ada"), "42 Ada")
	if bad {
		panic("a Go format row is wrong")
	}
	fmt.Println("ok")
}
