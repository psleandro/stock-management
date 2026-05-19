package repositories

import (
	"orders-api/internal/models"

	"github.com/google/uuid"
)

type OrderRepository interface {
	List() ([]models.Order, error)
	GetByID(orderID uuid.UUID) (*models.Order, error)
	Create(newOrder models.CreateOrderRequest) (*models.Order, error)
}
