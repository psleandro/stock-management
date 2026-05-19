package models

import (
	"time"

	"github.com/google/uuid"
)

type Order struct {
	ID        uuid.UUID   `gorm:"type:uuid;default:uuidv7();primaryKey"`
	Items     []OrderItem `gorm:"foreignKey:OrderID"`
	CreatedAt time.Time   `gorm:"autoCreateTime"`
}

type OrderItem struct {
	ID        uuid.UUID `gorm:"type:uuid;default:uuidv7();primaryKey"`
	OrderID   uuid.UUID `gorm:"type:uuid;not null;index"`
	ProductID uuid.UUID `gorm:"type:uuid;not null"`
	Place     string
	Quantity  int
}

type CreateOrderRequest struct {
	Items []OrderItem
}
