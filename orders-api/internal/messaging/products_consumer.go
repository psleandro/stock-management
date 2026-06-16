package messaging

import (
	"context"
	"encoding/json"
	"errors"
	"log"
	"orders-api/internal/models"
	"orders-api/internal/usecases"

	"github.com/google/uuid"
	"github.com/segmentio/kafka-go"
)

type ProductsConsumer struct {
	reader          *kafka.Reader
	productsUsecase *usecases.ProductUsecase
}

func NewProductsConsumer(brokers []string, productsUsecase *usecases.ProductUsecase) *ProductsConsumer {
	reader := kafka.NewReader(kafka.ReaderConfig{
		Brokers: brokers,
		Topic:   "products.events",
	})

	return &ProductsConsumer{
		reader:          reader,
		productsUsecase: productsUsecase,
	}
}

func (c *ProductsConsumer) InitializeConsume(ctx context.Context) {
	for {
		message, err := c.reader.ReadMessage(ctx)

		if err != nil {
			if errors.Is(err, context.Canceled) {
				return
			}

			log.Println(err)
			continue
		}

		var event models.ProductEvent
		json.Unmarshal(message.Value, &event)

		switch event.EventType {
		case "ProductCreated":
			c.productsUsecase.Upsert(event.Data)
		case "ProductUpdated":
			c.productsUsecase.Upsert(event.Data)
		case "ProductDeleted":
			ProductID, err := uuid.Parse(event.Data.ID)

			if err != nil {
				log.Printf("failed to parse id: %v", err)
				return
			}

			c.productsUsecase.Delete(ProductID)
		}
	}
}

func (c *ProductsConsumer) Close() {
	c.reader.Close()
}
