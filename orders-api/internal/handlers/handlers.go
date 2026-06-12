package handlers

import (
	"net/http"
	"orders-api/internal/usecases"
)

type Handlers struct {
	orderUsecase *usecases.OrderUsecase
}

func New(orderUsecase *usecases.OrderUsecase) *Handlers {
	return &Handlers{
		orderUsecase: orderUsecase,
	}
}

func (h Handlers) RegisterEndpoints(mux *http.ServeMux) {
	h.registerOrderEndpoints(mux)
}
