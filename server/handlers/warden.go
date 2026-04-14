package handlers

import (
	"log"

	"github.com/gorilla/websocket"
)

func HandleWarden(conn *websocket.Conn) {

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
