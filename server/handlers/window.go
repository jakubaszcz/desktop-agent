package handlers

import (
	"encoding/json"
	"fmt"
	"log"
	"os/exec"
	"runtime"
	"server/structures"
	"strings"

	"github.com/gorilla/websocket"
)

var windowConn *websocket.Conn
var windowLauching bool

func HandleWindow(conn *websocket.Conn) {

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
			log.Println("heartbeat from window")
		case "command":
			parts := strings.SplitN(msg.Action, ":", 2)

			if len(parts) != 2 {
				return
			}

			target := parts[0]
			command := parts[1]

			if fn, ok := Senders[target]; ok {
				fn("window", command)
			} else {
				return
			}
		}

		conn.WriteMessage(websocket.TextMessage, raw)
	}
}

func GetOSInterface() string {
	switch runtime.GOOS {
	case "windows":
		return "./interface.exe"
	case "darwin":
		return "./interface.app"
	case "linux":
		return "./interface"
	default:
		return "./interface"
	}
}

func SendToWindow(from string, cmd string) {
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

	if windowConn != nil {
		windowConn.WriteMessage(websocket.TextMessage, jsonBytes)
	}
}

func SendToResponseWindow(from string, cmd string, status string) {
	msg := Message{
		Type:   "response",
		From:   from,
		Data:   cmd,
		Status: status,
	}

	jsonBytes, err := json.Marshal(msg)
	if err != nil {
		fmt.Println("Erreur JSON:", err)
		return
	}

	if windowConn != nil {
		windowConn.WriteMessage(websocket.TextMessage, jsonBytes)
	}
}

func LaunchWindow() {
	if windowConn != nil || windowLauching {
		return
	}

	windowLauching = true
	go func() {
		cmd := exec.Command(GetOSInterface())
		start := cmd.Start()
		if start != nil {
			return
		}
		wait := cmd.Wait()
		if wait != nil {
			return
		}
		windowLauching = false
	}()
}
