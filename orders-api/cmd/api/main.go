package main

import (
	"fmt"
	"log"
	"orders-api/internal/handlers"
	"orders-api/internal/models"
	pgRepo "orders-api/internal/repositories/postgres"
	"orders-api/internal/usecases"
	"orders-api/pkg/config"
	"strconv"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

func main() {
	cfg := config.Load()

	dsn := fmt.Sprintf("host=%s port=%s user=%s password=%s dbname=%s  sslmode=disable",
		cfg.PostgresHost, cfg.PostgresPort, cfg.PostgresUser, cfg.PostgresPass, cfg.PostgresDB,
	)

	db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})
	if err != nil {
		log.Fatalf("Failed to connect to postgres: %v", err)
	}

	if err := db.AutoMigrate(&models.Order{}, &models.OrderItem{}); err != nil {
		log.Fatalf("Failed to migrate: %v", err)
	}

	orderRepository := pgRepo.NewOrderRepository(db)

	orderUsecase := usecases.NewOrderUsecase(orderRepository)

	h := handlers.New(orderUsecase)

	port, err := strconv.Atoi(cfg.ApiPort)
	if err != nil {
		log.Fatalf("invalid API port: %v", err)
	}

	h.Listen(port)
}
