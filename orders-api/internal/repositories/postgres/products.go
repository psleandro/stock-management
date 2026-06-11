package pgRepo

import (
	"orders-api/internal/models"
	"orders-api/internal/repositories"

	"github.com/google/uuid"
	"gorm.io/gorm"
	"gorm.io/gorm/clause"
)

type ProductRepository struct {
	db *gorm.DB
}

func NewProductRepository(db *gorm.DB) repositories.ProductRepository {
	return &ProductRepository{db}
}

func (repo *ProductRepository) Upsert(product models.UpsertProductRequest) (*models.Product, error) {
	productID, err := uuid.Parse(product.ID)

	if err != nil {
		return nil, err
	}

	product_model := models.Product{
		ID:          productID,
		Name:        product.Name,
		Brand:       product.Brand,
		Observation: product.Observation,
		CreatedAt:   product.CreatedAt,
		UpdatedAt:   product.UpdatedAt,
	}

	repo.db.Clauses(clause.OnConflict{
		Columns:   []clause.Column{{Name: "id"}},
		DoUpdates: clause.AssignmentColumns([]string{"name", "brand", "observation", "created_at", "updated_at"}),
	}).Create(&product_model)

	return &product_model, nil
}

func (repo *ProductRepository) Delete(productID uuid.UUID) (bool, error) {
	product := models.Product{
		ID: productID,
	}

	result := repo.db.Delete(&product)

	if result.Error != nil {
		return false, result.Error
	}

	return result.RowsAffected > 0, nil
}
