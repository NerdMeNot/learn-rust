package main

import "fmt"

type Job struct {
	name string
	log  *[]string // shared with main
}

func run(job Job) {
	*job.log = append(*job.log, job.name)
}

func main() {
	log := []string{}
	job := Job{"resize", &log}
	done := make(chan struct{})
	go func() {
		run(job)
		close(done)
	}()
	<-done
	fmt.Printf("log: %q\n", log)
}
