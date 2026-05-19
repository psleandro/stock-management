package pgRepo

import (
	"orders-api/internal/models"
	"orders-api/internal/repositories"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

type OrderRepository struct {
	db *gorm.DB
}

func NewOrderRepository(db *gorm.DB) repositories.OrderRepository {
	return &OrderRepository{db}
}

func (r OrderRepository) List() ([]models.Order, error) {
	var orders []models.Order

	if err := r.db.Preload("Items").Find(&orders).Error; err != nil {
		return nil, err
	}

	return orders, nil
}

func (r OrderRepository) GetByID(orderId uuid.UUID) (*models.Order, error) {
	order := models.Order{
		ID: orderId,
	}

	if err := r.db.Preload("Items").First(&order).Error; err != nil {
		return nil, err
	}

	return &order, nil
}

func (o *OrderRepository) Create(newOrder models.CreateOrderRequest) (*models.Order, error) {
	order := models.Order{
		Items: newOrder.Items,
	}

	err := o.db.Create(&order).Error

	if err != nil {
		return nil, err
	}

	return &order, nil
}
