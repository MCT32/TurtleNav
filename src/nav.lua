local ws = assert(http.websocket("ws://127.0.0.1:1337"))

ws.send("Hello!")
print(ws.receive())
ws.close()
