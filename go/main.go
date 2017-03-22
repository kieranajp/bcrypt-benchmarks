package main

import (
	"os"

	"golang.org/x/crypto/bcrypt"
)

func main() {
	hashedPassword := "$2y$10$VocDgtIzt4xuq.mH4o1YJe0UJscyf/EuEMW84Bq35eVJXz685ApUO"
	password := "abc123"
	err := bcrypt.CompareHashAndPassword([]byte(hashedPassword), []byte(password))
	if err != nil {
		os.Exit(1)
	}
}
