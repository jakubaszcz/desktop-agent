package main

import (
	"server/routes"
)

func main() {

	// Routes
	{
		routes.Machine()
		routes.Warden()
		routes.Window()
	}

	// Server
	routes.Server()
}
