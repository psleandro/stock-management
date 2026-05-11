package orders

import "orders-api/internal/models"

type Orders struct {
	orders []models.Order
}

func New() *Orders {
	return &Orders{orders: make([]models.Order,0)}
}	

func (o Orders) GetAll() []models.Order {
	return o.orders
}

func (o Orders) Get(orderId string) models.Order {
	return o.orders[0]
}

func (o *Orders) Add(newOrder models.Order ) {
	o.orders = append(o.orders, newOrder)
}