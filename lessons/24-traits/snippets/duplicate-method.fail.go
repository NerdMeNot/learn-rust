package main

type Vehicle interface{ Start() string }
type Appliance interface{ Start() string }

type Car struct{}

func (Car) Start() string { return "Vehicle::Start" }
func (Car) Start() string { return "Appliance::Start" }

func main() {}
