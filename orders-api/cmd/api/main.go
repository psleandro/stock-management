package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"orders-api/internal/handlers"
	"orders-api/internal/messaging"
	"orders-api/internal/models"
	pgRepo "orders-api/internal/repositories/postgres"
	"orders-api/internal/usecases"
	"orders-api/pkg/config"
	"os/signal"
	"strconv"
	"syscall"
	"time"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

func main() {
	cfg := config.Load()

	ctx, stop := signal.NotifyContext(
		context.Background(),
		syscall.SIGINT,
		syscall.SIGTERM,
	)

	defer stop()

	dsn := fmt.Sprintf("host=%s port=%s user=%s password=%s dbname=%s  sslmode=disable",
		cfg.PostgresHost, cfg.PostgresPort, cfg.PostgresUser, cfg.PostgresPass, cfg.PostgresDB,
	)

	db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})
	if err != nil {
		log.Fatalf("Failed to connect to postgres: %v", err)
	}

	if err := db.AutoMigrate(&models.Product{}, &models.Order{}, &models.OrderItem{}); err != nil {
		log.Fatalf("Failed to migrate: %v", err)
	}

	orderRepository := pgRepo.NewOrderRepository(db)

	orderUsecase := usecases.NewOrderUsecase(orderRepository)

	productRepository := pgRepo.NewProductRepository(db)

	productUsecase := usecases.NewProductUsecase(productRepository)

	products_consumer := messaging.NewProductsConsumer([]string{cfg.KafkaBrokers}, productUsecase)

	go products_consumer.InitializeConsume(ctx)

	defer products_consumer.Close()

	mux := http.NewServeMux()

	port, err := strconv.Atoi(cfg.ApiPort)
	if err != nil {
		log.Fatalf("invalid API port: %v", err)
	}

	h := handlers.New(orderUsecase)
	h.RegisterEndpoints(mux)

	srv := &http.Server{
		Addr:    fmt.Sprintf(":%d", port),
		Handler: mux,
	}

	go func() {
		<-ctx.Done()
		log.Println("shutting down in 5 seconds...")

		shutdownCtx, cancel := context.WithTimeout(
			context.Background(),
			5*time.Second,
		)

		defer cancel()

		srv.Shutdown(shutdownCtx)
	}()

	log.Println("Listening on ", "port", port)

	if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
		log.Fatalf("HTTP server ListenAndServe: %v", err)
	}
}
