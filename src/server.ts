import * as http from "http";
import * as fs from "fs";
import * as path from "path";
import { WebSocketServer } from "ws";

const theServer = http.createServer((req, res) => {
    if (req.url === "/" && req.method == "GET") {
        fs.readFile(path.join(__dirname, "index.html"), "utf-8", (err, data) => {
            if (err) {
                res.writeHead(500, { "Content-Type": "text/plain" });
                res.end("Internal Server Error");
                return;
            }
            res.writeHead(200, { "Content-Type": "text/html" })
            res.end(data);
        });
    } else if (req.url === "/client.js" && req.method === "GET") {

    }
});


