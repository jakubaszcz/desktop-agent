package handlers

import (
	"encoding/json"
	"log"
	"server/structures"

	"github.com/gorilla/websocket"
)

func HandleMachine(conn *websocket.Conn) {

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
			log.Println("heartbeat from machine")
		case "keybind":
			LaunchWindow()
		}

		conn.WriteMessage(websocket.TextMessage, raw)
	}
}
