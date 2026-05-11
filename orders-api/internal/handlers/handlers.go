package handlers

import (
	"fmt"
	"net/http"
	"orders-api/internal/usecases"
)

type Handlers struct {
	useCases *usecases.Usecases
}

func New (useCases *usecases.Usecases) *Handlers {
	return &Handlers{}
}

func (h Handlers) Listen (port int) error {
	return http.ListenAndServe(
		fmt.Sprintf(":%v", port),
		nil,
	)
}