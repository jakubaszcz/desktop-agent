package handlers

import (
	"log"
	"os/exec"
	"runtime"

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
		_, msg, err := conn.ReadMessage()
		if err != nil {
			break
		}

		log.Println(string(msg))

		conn.WriteMessage(websocket.TextMessage, []byte("heartbeat"))
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
