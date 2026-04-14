package main

import (
	"log"
	"net/http"

	"github.com/gorilla/websocket"
)

var windowConn *websocket.Conn

var upgrader = websocket.Upgrader{}

func handleWindow(conn *websocket.Conn) {

	windowConn = conn

	// The defer function assure when the function return, this line is called
	defer func() {
		err := windowConn.Close()
		if err != nil {
			return
		}
		windowConn = nil
	}()

	for {
		_, msg, err := conn.ReadMessage()
		if err != nil {
			break
		}

		log.Println(string(msg))

		conn.WriteMessage(websocket.TextMessage, []byte("heartbeat"))
	}
}

func handleMachine(conn *websocket.Conn) {

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

func handleWarden(conn *websocket.Conn) {

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
		go handleMachine(conn)
	})
}

func warden() {
	http.HandleFunc("/warden", func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			return
		}

		// Create a Go for every client
		go handleWarden(conn)
	})
}

func window() {
	http.HandleFunc("/window", func(w http.ResponseWriter, r *http.Request) {
		conn, err := upgrader.Upgrade(w, r, nil)
		if err != nil {
			return
		}

		// Create a Go for every client
		go handleWindow(conn)
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
		window()
	}

	// Server
	server()
}
