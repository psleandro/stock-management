package models

import (
	"time"

	"github.com/google/uuid"
)

type OrderProduct struct {
	ProductId uuid.UUID
	Quantity int
}

type Order struct {
	ID uuid.UUID
	Products []OrderProduct
	CreatedAt time.Time
}

