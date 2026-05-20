package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

type Product struct {
	ID           uuid.UUID `gorm:"type:uuid;default:uuidv7();primaryKey"`
	Name         string
	Brand        string
	Observation  string
	CurrentStock int `gorm:"not null; default:0"`
	CreatedAt    time.Time
	UpdatedAt    time.Time      `gorm:"autoUpdateTime"`
	DeletedAt    gorm.DeletedAt `gorm:"index"`
}
