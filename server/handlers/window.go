package handlers

import (
	"encoding/json"
	"log"
	"os/exec"
	"runtime"
	"server/structures"
	"strings"

	"github.com/gorilla/websocket"
)

var windowConn *websocket.Conn
var windowLauching bool

var sender = map[string]func(string, string){
	"warden": SendToWarden,
}

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

			if fn, ok := sender[target]; ok {
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
