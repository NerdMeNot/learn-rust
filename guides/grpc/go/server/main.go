package main

import (
	"context"
	"fmt"
	"log"
	"net"

	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	pb "greeter/pb"
)

type greeter struct {
	pb.UnimplementedGreeterServer
}

func (greeter) SayHello(ctx context.Context, req *pb.HelloRequest) (*pb.HelloReply, error) {
	if req.GetName() == "" {
		return nil, status.Error(codes.InvalidArgument, "name is required")
	}
	return &pb.HelloReply{Message: "Hello, " + req.GetName() + "!"}, nil
}

func main() {
	lis, err := net.Listen("tcp", "127.0.0.1:50051")
	if err != nil {
		log.Fatal(err)
	}
	s := grpc.NewServer()
	pb.RegisterGreeterServer(s, greeter{})
	fmt.Printf("gRPC server listening on %s\n", lis.Addr())
	log.Fatal(s.Serve(lis))
}
