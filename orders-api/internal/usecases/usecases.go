package usecases

import (
	"orders-api/internal/models"
	"orders-api/internal/repositories"
	"time"

	"github.com/google/uuid"
)

type Usecases struct {
	repos *repositories.Repositories
}

func New(repos *repositories.Repositories) *Usecases {
	return & Usecases{
		repos: repos,
	}
}

func (u Usecases) GetAll() []models.Order {
	orders := u.repos.Order.GetAll()

	return orders
}

func (u Usecases) Get() models.Order {
	return u.repos.Order.Get("")
}

func (u Usecases) Add(newOrder models.CreateOrderRequest) (models.Order, error) {

	orderId, err := uuid.NewV7()

	if err != nil {
		return models.Order{}, err
	}

	orderPayload := models.Order {
		ID: orderId,
		Products: newOrder.Products,
		CreatedAt: time.Now(),
	}

	u.repos.Order.Add(orderPayload)

	return orderPayload, nil
}