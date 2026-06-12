package handlers

import (
	"encoding/json"
	"net/http"

	"orders-api/internal/models"

	"github.com/google/uuid"
)

func (h Handlers) registerOrderEndpoints(mux *http.ServeMux) {
	mux.HandleFunc("GET /orders", h.getAllOrders)
	mux.HandleFunc("GET /orders/{orderID}", h.getOrderByID)
	mux.HandleFunc("POST /orders", h.createOrder)
}

func (h Handlers) getAllOrders(w http.ResponseWriter, r *http.Request) {
	orders, err := h.orderUsecase.List()

	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		json.NewEncoder(w).Encode(models.ErrorResponse{Reason: err.Error()})
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(orders)
}

func (h Handlers) getOrderByID(w http.ResponseWriter, r *http.Request) {
	orderIDStr := r.PathValue("orderID")

	orderID, err := uuid.Parse((orderIDStr))

	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode(models.ErrorResponse{Reason: err.Error()})
		return
	}

	order, err := h.orderUsecase.GetByID(orderID)

	if err != nil {
		w.WriteHeader(http.StatusNotFound)
		json.NewEncoder(w).Encode(models.ErrorResponse{Reason: err.Error()})
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(order)
}

func (h Handlers) createOrder(w http.ResponseWriter, r *http.Request) {
	var req models.CreateOrderRequest

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		w.WriteHeader(http.StatusBadRequest)
		json.NewEncoder(w).Encode(models.ErrorResponse{Reason: err.Error()})
		return
	}

	createdOrder, err := h.orderUsecase.Add(req)

	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		json.NewEncoder(w).Encode(models.ErrorResponse{Reason: err.Error()})
		return
	}

	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(createdOrder)
}
