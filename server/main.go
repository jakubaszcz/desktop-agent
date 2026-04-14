package main

import (
	"encoding/json"
	"log"
	"net/http"
	"os/exec"

	"github.com/gorilla/websocket"
)

type Message struct {
	Type   string `json:"type"`
	Action string `json:"action"`
}

var windowConn *websocket.Conn
var windowLauching bool

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
		_, raw, err := conn.ReadMessage()
		if err != nil {
			break
		}

		var msg Message
		err = json.Unmarshal(raw, &msg)
		if err != nil {
			log.Println(err)
		}
		switch msg.Type {
		case "heartbeat":
			log.Println("heartbeat from machine")
		case "keybind":
			launchWindow()
		}

		conn.WriteMessage(websocket.TextMessage, raw)
	}
}

func launchWindow() {
	if windowConn != nil || windowLauching {
		return
	}

	windowLauching = true
	go func() {
		cmd := exec.Command("./interface.exe")
		cmd.Start()
		cmd.Wait()
		windowLauching = false
	}()
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
