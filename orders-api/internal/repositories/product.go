package repositories

import (
	"orders-api/internal/models"

	"github.com/google/uuid"
)

type ProductRepository interface {
	Upsert(product models.UpsertProductRequest) (*models.Product, error)
	Delete(productID uuid.UUID) (bool, error)
}
