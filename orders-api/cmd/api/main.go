package main

import (
	"log"
	"orders-api/internal/handlers"
	memoryRepo "orders-api/internal/repositories/memory"
	"orders-api/internal/usecases"
	"orders-api/pkg/config"
	"strconv"
)

func main() {
	cfg := config.Load()

	orderRepository := memoryRepo.NewOrderRepository()

	orderUsecase := usecases.NewOrderUsecase(orderRepository)

	h := handlers.New(orderUsecase)

	port, err := strconv.Atoi(cfg.ApiPort)
	if err != nil {
		log.Fatalf("invalid API port: %v", err)
	}

	h.Listen(port)
}
