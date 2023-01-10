package main

import (
	"github.com/spf13/cobra"
	"github.com/things.over.internet/thing.manager/internal/consumers"
	"github.com/things.over.internet/thing.manager/internal/grpc"
)

func main() {
	root := &cobra.Command{
		Use:   "things-manager",
		Short: "IoT Things Manager",
	}

	root.AddCommand(consumers.Cmd(), grpc.Cmd())

	root.Execute()
}
