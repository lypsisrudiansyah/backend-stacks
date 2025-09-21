package main

import (
	"fmt"
	"log"
	"net/http"
	"sync"
)

var counter int
var mutex = &sync.Mutex{}

func incrementCounter(w http.ResponseWriter, r *http.Request) {
	log.Printf("%s %s", r.Method, r.URL.Path)

	if r.URL.Path != "/" {
		http.NotFound(w, r)
		return
	}
	mutex.Lock()
	defer mutex.Unlock()
	counter++
	fmt.Fprintf(w, "Counter value: %d\n", counter)
}

func main() {
	http.HandleFunc("/", incrementCounter)
	http.HandleFunc("/favicon.ico", func(w http.ResponseWriter, r *http.Request) {
		log.Println("Favicon requested")
		http.NotFound(w, r) // or serve a real icon
	})
	fmt.Println("Server running at http://localhost:8080")
	http.ListenAndServe(":8080", nil)
	// err := http.ListenAndServe(":8080", nil)
	// if err != nil {
	// 	log.Fatal(err)
	// }
}
