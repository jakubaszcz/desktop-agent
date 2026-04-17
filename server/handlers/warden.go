package handlers

import (
	"encoding/json"
	"fmt"
	"log"

	"github.com/gorilla/websocket"
)

var wardenConn *websocket.Conn

type Message struct {
	Type string `json:"type"`
	From string `json:"from"`
	Data string `json:"data,omitempty"`
}

func HandleWarden(conn *websocket.Conn) {

	wardenConn = conn

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

func SendToWarden(from string, cmd string) {
	msg := Message{
		Type: "command",
		From: from,
		Data: cmd,
	}

	jsonBytes, err := json.Marshal(msg)
	if err != nil {
		fmt.Println("Erreur JSON:", err)
		return
	}

	wardenConn.WriteMessage(websocket.TextMessage, jsonBytes)
}
