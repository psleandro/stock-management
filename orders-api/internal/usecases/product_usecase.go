package usecases

import (
	"orders-api/internal/models"
	"orders-api/internal/repositories"

	"github.com/google/uuid"
)

type ProductUsecase struct {
	repo repositories.ProductRepository
}

func NewProductUsecase(repo repositories.ProductRepository) *ProductUsecase {
	return &ProductUsecase{
		repo: repo,
	}
}

func (u *ProductUsecase) Upsert(product models.UpsertProductRequest) (*models.Product, error) {
	return u.repo.Upsert(product)
}

func (u *ProductUsecase) Delete(productID uuid.UUID) (bool, error) {
	return u.repo.Delete(productID)
}
