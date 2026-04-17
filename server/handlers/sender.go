package handlers

var Senders = map[string]func(string, string){
	"warden": SendToWarden,
	"window": SendToWindow,
}
