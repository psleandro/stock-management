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
	CurrentStock int            `gorm:"not null; default:0"`
	CreatedAt    time.Time      `gorm:"autoCreateTime:false"`
	UpdatedAt    time.Time      `gorm:"autoUpdateTime:false"`
	SyncedAt     time.Time      `gorm:"autoUpdateTime"`
	DeletedAt    gorm.DeletedAt `gorm:"index"`
}

type UpsertProductRequest struct {
	ID          string    `json:"id"`
	Name        string    `json:"name"`
	Brand       string    `json:"brand"`
	Observation string    `json:"observation"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
}

type ProductEvent struct {
	EventType string               `json:"event_type"`
	Data      UpsertProductRequest `json:"data"`
}
