package pkg

import (
	"log"

	"github.com/spf13/cobra"
	"go.uber.org/dig"
	"go.uber.org/zap"
)

type SubContainer func(*dig.Container)

type CommonArgs struct {
	dig.In

	Logger zap.Logger
}

func Commander(main any, subs ...SubContainer) func(*cobra.Command, []string) {
	return func(cmd *cobra.Command, s []string) {
		//cfg

		container := NewContainer()

		container.Provide(func() *cobra.Command { return cmd })

		for _, sub := range subs {
			sub(container)
		}

		if err := container.Invoke(main); err != nil {
			log.Panic("application error")
		}
	}
}
