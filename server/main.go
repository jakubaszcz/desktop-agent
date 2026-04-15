package main

import (
	"server/routes"

	"github.com/getlantern/systray"
)

func onReady() {
	systray.SetTitle("Desktop agent")
	systray.SetTooltip("Desktop agent")

	// Create the button to shut down the server
	mShutDown := systray.AddMenuItem("Shutdown", "Shutdown the desktop agent")

	go func() {
		for {
			select {
			case <-mShutDown.ClickedCh:
				systray.Quit()
				return
			}
		}
	}()
}

func onExit() {
	// Empty
}

func main() {

	// Routes
	{
		routes.Machine()
		routes.Warden()
		routes.Window()
	}

	// Create the stray to close the desktop agent
	systray.Run(onReady, onExit)

	// Server
	routes.Server()

}
