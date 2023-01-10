package grpc

import (
	"github.com/spf13/cobra"
	"github.com/things.over.internet/thing.manager/pkg"
)

type GrpcServerArgs struct {
	pkg.CommonArgs
}

func GrpcServer(args GrpcServerArgs) {
	println("doing something")
}

func Cmd() *cobra.Command {
	return &cobra.Command{
		Use:   "server",
		Short: "gRPC Server",
		Run:   pkg.Commander(GrpcServer, GrpcServerContainer),
	}
}
