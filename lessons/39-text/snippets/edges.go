package main

import (
	"fmt"
	"path/filepath"
	"strconv"
	"strings"
)

func main() {
	fmt.Printf("Split(\"a,b,\", \",\"):  %q\n", strings.Split("a,b,", ","))
	fmt.Printf("Split(\"\", \",\"):      %q\n", strings.Split("", ","))
	fmt.Printf("Fields(\" a  b\\t\"):    %q\n", strings.Fields(" a  b\t"))
	fmt.Printf("Fields(\"\"):           %q\n", strings.Fields(""))
	fmt.Printf("SplitN(\"k=v=w\", \"=\", 2): %q\n", strings.SplitN("k=v=w", "=", 2))

	// Cut is split_once. TrimLeft takes a SET of characters, TrimPrefix a prefix.
	before, after, found := strings.Cut("k=v=w", "=")
	fmt.Println("Cut:", before, after, found)
	fmt.Printf("TrimLeft(\"///a\", \"/\"): %q\n", strings.TrimLeft("///a", "/"))
	fmt.Printf("TrimPrefix(\"///a\", \"/\"): %q\n", strings.TrimPrefix("///a", "/"))

	fmt.Println("Replace(\"a-a-a\", \"-\", \"+\", 1):", strings.Replace("a-a-a", "-", "+", 1))
	fmt.Println("ReplaceAll(\"a-a-a\", \"-\", \"+\"):", strings.ReplaceAll("a-a-a", "-", "+"))
	fmt.Printf("Ext(\".bashrc\"): %q, Ext(\"a.tar.gz\"): %q\n", filepath.Ext(".bashrc"), filepath.Ext("a.tar.gz"))

	for _, text := range []string{"42", "+42", " 42", "1_000", "٣", "3000000000"} {
		n, err := strconv.Atoi(text)
		fmt.Printf("Atoi(%q): %d, %v\n", text, n, err)
	}

	fmt.Println("ToUpper(\"straße\"):", strings.ToUpper("straße"))
	fmt.Println("EqualFold(\"É\", \"é\"):", strings.EqualFold("É", "é"))
}
