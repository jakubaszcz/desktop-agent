package handlers

import (
	"encoding/json"
	"fmt"
	"log"
	"server/structures"
	"strings"

	"github.com/gorilla/websocket"
)

var wardenConn *websocket.Conn

type Message struct {
	Type   string `json:"type"`
	From   string `json:"from"`
	Data   string `json:"data,omitempty"`
	Status string `json:"status,omitempty"`
}

func HandleWarden(conn *websocket.Conn) {

	wardenConn = conn

	// The defer function assure when the function return, this line is called
	defer conn.Close()

	for {
		_, raw, err := conn.ReadMessage()
		if err != nil {
			break
		}

		var msg structures.Message
		err = json.Unmarshal(raw, &msg)
		if err != nil {
			log.Println(err)
		}
		switch msg.Type {
		case "heartbeat":
			log.Println("heartbeat from warden")
		case "command":
			parts := strings.SplitN(msg.Action, ":", 2)

			if len(parts) != 2 {
				return
			}

			target := parts[0]
			command := parts[1]

			if fn, ok := Senders[target]; ok {
				fn("warden", command)
			} else {
				return
			}
		case "response":
			parts := strings.SplitN(msg.Action, ":", 3)

			if len(parts) != 3 {
				return
			}

			target := parts[0]
			command := parts[1]
			status := parts[2]

			if fn, ok := ResponseSenders[target]; ok {
				fn("warden", command, status)
			} else {
				return
			}
		}

		conn.WriteMessage(websocket.TextMessage, raw)
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
