package handlers

import (
	"fmt"
	"log/slog"
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

func (h Handlers) Listen(port int) error {
	h.registerOrderEndpoints()

	slog.Info("Listening on ", "port", port)

	return http.ListenAndServe(
		fmt.Sprintf(":%v", port),
		nil,
	)
}
