package usecases

import (
	"orders-api/internal/models"
	"orders-api/internal/repositories"

	"github.com/google/uuid"
)

type OrderUsecase struct {
	repo repositories.OrderRepository
}

func NewOrderUsecase(repo repositories.OrderRepository) *OrderUsecase {
	return &OrderUsecase{
		repo: repo,
	}
}

func (u OrderUsecase) List() ([]models.Order, error) {
	orders, err := u.repo.List()

	if err != nil {
		return nil, err
	}

	return orders, nil
}

func (u OrderUsecase) GetByID(orderID uuid.UUID) (*models.Order, error) {
	order, err := u.repo.GetByID(orderID)

	if err != nil {
		return nil, err
	}

	return order, nil
}

func (u OrderUsecase) Add(newOrder models.CreateOrderRequest) (*models.Order, error) {
	order, err := u.repo.Create(newOrder)

	return order, err
}
