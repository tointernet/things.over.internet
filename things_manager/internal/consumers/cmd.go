package consumers

import (
	"github.com/spf13/cobra"
	"github.com/things.over.internet/thing.manager/pkg"
)

type ConsumersArgs struct {
	pkg.CommonArgs
}

func Consumers(args ConsumersArgs) {
	println("doing something")
}

func Cmd() *cobra.Command {
	return &cobra.Command{
		Use:   "consumers",
		Short: "RabbitMQ consumers",
		Run:   pkg.Commander(Consumers, ConsumersContainer),
	}
}
