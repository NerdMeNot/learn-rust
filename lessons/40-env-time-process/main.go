package main

import (
	"fmt"
	"os"
	"os/exec"
	"time"
)

func main() {
	fmt.Println(len(os.Args)-1, "arg(s) after the program name")

	port := os.Getenv("RBC_PORT")
	if port == "" {
		port = "8080"
	}
	fmt.Println("port", port)
	if v, ok := os.LookupEnv("RBC_MISSING"); ok {
		fmt.Println("RBC_MISSING=" + v)
	} else {
		fmt.Println("RBC_MISSING: not set")
	}

	start := time.Now()
	time.Sleep(20 * time.Millisecond)
	took := time.Since(start)
	fmt.Println("slept at least 20ms:", took >= 20*time.Millisecond)

	out, err := exec.Command("echo", "hello from echo").Output()
	if err != nil {
		panic(err)
	}
	fmt.Print(string(out))
	fmt.Println("exit ok:", err == nil)
}
