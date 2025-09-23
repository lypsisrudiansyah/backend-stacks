import * as http from "http";
import * as fs from "fs";
import * as path from "path";
import { WebSocketServer } from "ws";

const theServer = http.createServer((req, res) => {
    if (req.url === "/" && req.method == "GET") {
        fs.readFile(path.join(__dirname, "index.html"), "utf-8", (err, data) => {

        });
    }
});


