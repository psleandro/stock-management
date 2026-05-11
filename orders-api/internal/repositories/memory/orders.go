package memoryRepo

import (
	"errors"
	"orders-api/internal/models"
	"orders-api/internal/repositories"
	"slices"
	"time"

	"github.com/google/uuid"
)

type OrderRepository struct {
	orders []models.Order
}

func NewOrderRepository() repositories.OrderRepository {
	return &OrderRepository{orders: make([]models.Order, 0)}
}

func (o OrderRepository) List() ([]models.Order, error) {
	return o.orders, nil
}

func (o OrderRepository) GetByID(orderId string) (*models.Order, error) {
	orderIdx := slices.IndexFunc(o.orders, func(o models.Order) bool {
		return o.ID.String() == orderId
	})

	if orderIdx == -1 {
		return nil, errors.New("Order not found!")
	}

	return &o.orders[orderIdx], nil
}

func (o *OrderRepository) Create(newOrder models.CreateOrderRequest) (*models.Order, error) {
	orderId, err := uuid.NewV7()

	if err != nil {
		return nil, err
	}

	order := models.Order{
		ID:        orderId,
		Items:     newOrder.Items,
		CreatedAt: time.Now(),
	}

	o.orders = append(o.orders, order)

	return &order, nil
}
