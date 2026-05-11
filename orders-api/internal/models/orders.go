package models

import (
	"time"

	"github.com/google/uuid"
)

type OrderItem struct {
	ProductId uuid.UUID
	Place     string
	Quantity  int
}

type Order struct {
	ID        uuid.UUID
	Items     []OrderItem
	CreatedAt time.Time
}

type CreateOrderRequest struct {
	Items []OrderItem
}
