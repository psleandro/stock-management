package main

import (
	"orders-api/internal/handlers"
	"orders-api/internal/repositories"
	"orders-api/internal/usecases"
)

func main(){
	repos := repositories.New()

	useCases := usecases.New(repos)

	h := handlers.New(useCases)

	h.Listen(8001)
}