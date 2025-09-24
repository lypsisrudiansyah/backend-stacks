const socket = new WebSocket(":8080")

socket.addEventListener("open", () => {
    console.log("Server Websocket - Connected!");
    socket.send("Initial into the Socket!")
})

socket.addEventListener("message", (event: MessageEvent) => {
    console.log("Message from server: ", event.data);
})
