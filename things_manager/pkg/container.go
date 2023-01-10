package pkg

import "go.uber.org/dig"

func NewContainer() *dig.Container {
	container := dig.New()

	container.Provide(provideLogger)

	return container
}

func provideLogger() {}
