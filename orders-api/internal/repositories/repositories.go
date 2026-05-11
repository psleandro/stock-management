package repositories

import (
	"orders-api/internal/models"
	"orders-api/internal/repositories/orders"
)

type Repositories struct {
	Order interface {
		GetAll() []models.Order
		Get(orderId string) models.Order
		Add (newOrder models.Order)
	}
}

func New() *Repositories {
	return &Repositories{
		Order: orders.New(),
	}
}