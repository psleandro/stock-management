// package models

// import (
// 	"time"

// 	"github.com/google/uuid"
// )

// type Product struct {
// 	ID uuid.UUID
// 	Name string
// }

// type OrderProduct struct {
// 	ProductId uuid.UUID
// 	ProductName string

// 	Quantity int
// }

// type Order struct {
// 	ID uuid.UUID
// 	Products []OrderProduct
// 	CreatedAt time.Time
// }


// type OrderRequestProduct struct {
// 	ProductId uuid.UUID
// 	Quantity int
// }

// type OrderRequest struct {
// 	// ID uuid.UUID
// 	Products []OrderRequestProduct
// }