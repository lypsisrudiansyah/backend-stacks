const socket = new WebSocket(":8080")

socket.addEventListener("open", () => {
    console.log("Server Websocket - Connected!");
    socket.send("Initial into the Socket!")
})

socket.addEventListener("message", (event: MessageEvent) => {
    console.log("Message from server: ", event.data);
})

socket.addEventListener("close", () => {
    console.log("Disconnected from WebSocket !")
})

socket.addEventListener("error", (event: Event) => {
    console.error("websocket server error: ", event);
})

setInterval(() => {
    if (socket.readyState === WebSocket.OPEN) {
        socket.send("Ping from client!");
    }
}, 3500);