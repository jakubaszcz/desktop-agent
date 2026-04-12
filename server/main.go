package main

import (
	"log"
	"net/http"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{}

func handleClient(conn *websocket.Conn) {

	// The defer function assure when the function return, this line is called
	defer conn.Close()

	for {
		_, msg, err := conn.ReadMessage()
		if err != nil {
			break
		}

		log.Println(string(msg))

		conn.WriteMessage(websocket.TextMessage, []byte("heartbeat"))
	}
}

func machine() {
	http.HandleFunc("/machine", func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			return
		}

		// Create a Go for every client
		go handleClient(conn)
	})
}

func warden() {
	http.HandleFunc("/warden", func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			return
		}

		// Create a Go for every client
		go handleClient(conn)
	})
}

func server() {
	if err := http.ListenAndServe(":8080", nil); err != nil {
		log.Fatal(err)
	}

	log.Println("Listening on :8080...")
}

func main() {

	// Routes
	{
		machine()
		warden()
	}

	// Server
	server()
}
