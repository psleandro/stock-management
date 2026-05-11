package repositories

import (
	"orders-api/internal/models"
)

type OrderRepository interface {
	List() ([]models.Order, error)
	GetByID(orderID string) (*models.Order, error)
	Create(newOrder models.CreateOrderRequest) (*models.Order, error)
}
