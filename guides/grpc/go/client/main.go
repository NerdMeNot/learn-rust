package main

import (
	"context"
	"fmt"
	"log"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/status"

	pb "greeter/pb"
)

func main() {
	conn, err := grpc.NewClient("127.0.0.1:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatal(err)
	}
	defer conn.Close()

	// An ordinary variable: no `mut` in Go.
	client := pb.NewGreeterClient(conn)
	ctx := context.Background()

	reply, err := client.SayHello(ctx, &pb.HelloRequest{Name: "Ana"})
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("server said:", reply.GetMessage())

	_, err = client.SayHello(ctx, &pb.HelloRequest{}) // name is empty
	if err != nil {
		st := status.Convert(err)
		fmt.Printf("error %v: %s\n", st.Code(), st.Message())
	}
}
