package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	verbose := flag.Bool("verbose", false, "print more detail")
	flag.Parse()
	if flag.NArg() < 1 {
		fmt.Fprintln(os.Stderr, "usage: todo [-verbose] add|list ...")
		os.Exit(2)
	}

	switch flag.Arg(0) {
	case "add":
		addCmd := flag.NewFlagSet("add", flag.ExitOnError)
		priority := addCmd.Int("priority", 2, "priority from 1 (high) to 3 (low)")
		addCmd.Parse(flag.Args()[1:])
		if addCmd.NArg() < 1 {
			fmt.Fprintln(os.Stderr, "usage: todo add [-priority N] TEXT")
			os.Exit(2)
		}
		fmt.Printf("added %q with priority %d\n", addCmd.Arg(0), *priority)
		if *verbose {
			fmt.Println("(verbose mode is on)")
		}
	case "list":
		fmt.Println("nothing to list yet")
	default:
		fmt.Fprintln(os.Stderr, "unknown command:", flag.Arg(0))
		os.Exit(2)
	}
}
