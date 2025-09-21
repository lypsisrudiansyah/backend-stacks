package main

import (
	"fmt"
	"log"
	"net/http"
)

var counter int

func incrementCounter(w http.ResponseWriter, r *http.Request) {
	log.Printf("%s %s", r.Method, r.URL.Path)
	counter++

	if r.URL.Path != "/" {
		http.NotFound(w, r)
		return
	}

	fmt.Fprintf(w, "Counter value: %d\n", counter)
}

func main() {
	http.HandleFunc("/", incrementCounter)

	fmt.Println("Server running at http://localhost:8080")
	http.ListenAndServe(":8080", nil)
	// err := http.ListenAndServe(":8080", nil)
	// if err != nil {
	// 	log.Fatal(err)
	// }
}
