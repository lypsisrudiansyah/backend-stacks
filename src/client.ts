const socket = new WebSocket(":8080")
// const socket = new WebSocket("ws://localhost:8080")

const msgList = document.getElementById("messages") as HTMLUListElement;
const msgInput = document.getElementById("msgInput") as HTMLInputElement;
const sendBtn = document.getElementById("sendBtn") as HTMLButtonElement;

function addMessage(text: string) {
    const li = document.createElement("li");
    li.textContent = text;
    msgList.appendChild(li);
}

socket.addEventListener("open", () => {
    console.log("Server Websocket - Connected!");
    socket.send("Initial into the Socket!")
})

socket.addEventListener("message", (event: MessageEvent) => {
    addMessage("You : " + event.data)
    console.log("Message from server: ", event.data);
})

socket.addEventListener("close", () => {
    console.log("Disconnected from WebSocket !")
})

socket.addEventListener("error", (event: Event) => {
    console.error("websocket server error: ", event);
})

sendBtn.addEventListener("click", () => {
    const msg = msgInput.value.trim();
    if (msg && socket.readyState === WebSocket.OPEN) {
        socket.send(msg);
        // addMessage("You: " + msg);
        // msgInput.value = "";
    }
});

setInterval(() => {
    if (socket.readyState === WebSocket.OPEN) {
        socket.send("Periodic ping 3.5 sec To Server Socket!");
    }
}, 3500);