package routes

import (
	"log"
	"net/http"
	"server/handlers"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{}

func Machine() {
	http.HandleFunc("/machine", func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			return
		}

		// Create a Go for every client
		go handlers.HandleMachine(conn)
	})
}

func Warden() {
	http.HandleFunc("/warden", func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			return
		}

		// Create a Go for every client
		go handlers.HandleWarden(conn)
	})
}

func Window() {
	http.HandleFunc("/window", func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			return
		}

		// Create a Go for every client
		go handlers.HandleWindow(conn)
	})
}

func Server() {
	if err := http.ListenAndServe(":8080", nil); err != nil {
		log.Fatal(err)
	}

	log.Println("Listening on :8080...")
}
