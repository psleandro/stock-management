# Stock Management - API (Backend)

## 1. Project Overview

Stock-management API is a modern, high-performance inventory and stock management backend designed with a strict multi-tenant architecture.

It solves the problem of securely managing inventory across different organizations (workspaces) within a single deployed instance. The system allows clients to track products, manage suppliers, define physical storage places, and securely handle stock movements (entries and exits) while guaranteeing data isolation between different workspaces.

### Main Features

- **Multi-Tenancy**: Data is strictly isolated by `workspace_id`, ensuring users only access their organization's data.
- **Stock Movement Tracking**: Robust handling of stock entries (from suppliers) and exits (to places).
- **Inventory Protection**: Business rules prevent operations like withdrawing more stock than is currently available.
- **Secure Authentication**: JWT-based authentication with secure password hashing.
- **Event-Driven Integration**: Kafka-based event publishing for downstream synchronization.

---

## 2. Tech Stack

- **Language**: Rust (Edition 2024)
- **Web Framework**: Axum 0.8
- **Async Runtime**: Tokio
- **Database**: PostgreSQL 18+ (Required for native UUID v7 support)
- **ORM / Query Builder**: Diesel 2.3
- **Message Broker**: Apache Kafka
- **Connection Pooling**: deadpool-diesel
- **Authentication**: JWT (`jsonwebtoken`) & Password Hashing (`argon2` / `password-hash`)
- **Validation**: `validator` crate
- **Infrastructure**: Docker & Docker Compose

---

## 3. Architecture

The application follows a **Layered Architecture** (heavily inspired by Clean Architecture), enforcing strict separation of concerns.

### Folder Structure (`src/`)

- `handlers/`: HTTP layer. Responsible for parsing requests, extracting state/payloads, calling services, and formatting HTTP responses.
- `services/`: Core domain logic. Enforces business rules and orchestrates repository calls. It has no knowledge of HTTP or Axum.
- `infrastructure/db/`: Data persistence layer. Contains Repositories that use Diesel to interact with PostgreSQL.
- `infrastructure/messaging/`: Event publishing layer. Handles Kafka integration.
- `models/`: Domain entities and Data Transfer Objects (DTOs).
- `contracts/`: Interfaces and shared types (e.g., `EventBus`, `Event`).
- `extractors/`: Custom Axum extractors (e.g., `AuthenticatedUser`, `ValidatedJson`) for cross-cutting concerns like auth and request validation.
- `routes/`: Maps HTTP endpoints to their respective handlers.

---

## 4. Database Schema & Models

The system uses a relational schema designed for multi-tenant isolation. Almost all entities are tied to a `workspace_id`.

### Entity Descriptions

- **Users**: Authentication credentials and basic profile information.
- **Workspaces**: The top-level container for data isolation. Each workspace has an owner (User).
- **Products**: Inventory items defined within a workspace. Includes `base_unit` (Unit, Box, etc.) and `min_stock` alerts.
- **Places**: Physical storage locations (e.g., "Warehouse A", "Shelf 1") where stock can be moved.
- **Suppliers**: Vendors from whom products are purchased.
- **Stock Movements**: The ledger of all inventory changes. Tracks quantity, cost, and association with a supplier (entry) or place (exit).

### Schema Diagram

```mermaid
erDiagram
    USER ||--o{ WORKSPACE : owns
    WORKSPACE ||--o{ PRODUCT : contains
    WORKSPACE ||--o{ PLACE : contains
    WORKSPACE ||--o{ SUPPLIER : contains
    PRODUCT ||--o{ STOCK_MOVEMENT : recorded-in
    PLACE ||--o{ STOCK_MOVEMENT : receives-exit
    SUPPLIER ||--o{ STOCK_MOVEMENT : provides-entry

    USER {
        int id PK
        string email
        string password
    }
    WORKSPACE {
        int id PK
        string name
        int owner_id FK
    }
    PRODUCT {
        uuid id PK
        int workspace_id FK
        string name
        string base_unit
        int min_stock
    }
    STOCK_MOVEMENT {
        int id PK
        uuid product_id FK
        int supplier_id FK "optional (Entry)"
        int place_id FK "optional (Exit)"
        int quantity "Positive for entry, negative for exit"
        int unit_cost_in_cents
    }
```

### Constraints & Isolation

- **Multi-Tenancy**: All queries for products, places, suppliers, and movements must filter by `workspace_id`.
- **Soft Deletes**: Most tables include a `deleted_at` field for audit-friendly deletions.
- **Data Integrity**: Foreign keys ensure that a movement cannot exist without a product, and must be linked to either a supplier (entry) or a place (exit).
- **Primary Keys**: The system utilizes **UUID v7** for product identification. This requires **PostgreSQL 18** or higher to leverage native support for time-sortable, sequential UUIDs, which significantly improves indexing performance compared to random UUIDs.

---

## 5. Kafka Integration

The API publishes domain events to Kafka to allow other services to react to product changes.

### Published Events

Events are produced whenever a product is modified. The events are published to specific topics:

| Event               | Topic             | Key Format       |
| :------------------ | :---------------- | :--------------- |
| **Product Created** | `product_created` | `product#{uuid}` |
| **Product Updated** | `product_updated` | `product#{uuid}` |
| **Product Deleted** | `product_deleted` | `product#{uuid}` |

### Event Payload Example

The payload is a JSON representation of the `Product` model:

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "workspace_id": 1,
  "name": "Industrial Widget",
  "base_unit": "Unit",
  "brand": "Acme Corp",
  "min_stock": 10,
  "observation": "Fragile items",
  "created_at": "2024-05-08T10:00:00Z",
  "updated_at": "2024-05-08T10:00:00Z",
  "deleted_at": null
}
```

### Implementation Details

- **Producer**: Uses `rust-rdkafka` (based on `librdkafka`).
- **Serialization**: Payloads are serialized to JSON.
- **Delivery**: The application starts a background poller to handle delivery reports and ensure Kafka connectivity.

---

## 6. How to Run Locally

### Prerequisites

- [Docker & Docker Compose](https://www.docker.com/)
- [Rust / Cargo](https://rustup.rs/) (for local development and testing)

### Setup & Execution

1. Create your environment variables file based on the example (`.env.example`):
   ```bash
   cp api/.env.example api/.env
   ```
2. Run the API and database using Docker Compose:
   ```bash
   docker-compose up --build
   ```

---

## 7. Common Problems / Troubleshooting

### Docker vs. Localhost Connectivity

A common issue occurs when configuring connection strings for Database or Kafka:

- **When running with Docker**: Environment variables (like `DATABASE_URL` or `KAFKA_BROKERS`) must reference the **service name** defined in `docker-compose.yml` instead of `localhost`.
  - _Example_: `DATABASE_URL=postgres://user:pass@stock-management-db:5432/db`
- **Network Isolation**: All related containers (API, DB, Kafka) must belong to the same Docker network to communicate via service names.
- **When running locally (Standalone)**: If you are running the Rust binary via `cargo run` but keeping the database in Docker, you should use `localhost` because the ports are mapped to your host machine.

---

## 8. Testing

The API includes a comprehensive suite of integration and unit tests.

### Test Database Requirements

> [!IMPORTANT]
> Automated tests require a **dedicated test database** (configured via `DATABASE_URL_TEST`).

- **Why a dedicated DB?**: During test execution, the database will be:
  1. **Cleaned**: All tables are truncated before/after suites.
  2. **Seeded/Polluted**: Tests inject mock data that may violate standard business states to test edge cases.
  3. **Modified**: Multiple concurrent or sequential tests will modify state rapidly.
- **Warning**: **NEVER** use your production or development database for running tests. You will lose your data.

### How to Run

```bash
cargo test
```

---

## 9. Business Rules

The system implements several critical domain rules:

- **Strict Data Isolation (Multi-Tenancy)**: All services and repository methods require a `workspace_id`. Cross-workspace data access is structurally impossible.
- **Stock Exits**: When creating a stock exit, the system calculates the current stock for the requested product. If the exit quantity exceeds the available stock, a `NotEnoughStock` domain error is returned.
- **Data Integrity Constraints**:
  - A Stock Entry _must_ be associated with a valid Product and a valid Supplier.
  - A Stock Exit _must_ be associated with a valid Product and a valid Place.
- **Quantity Representation**: Entries are processed as positive quantities, while exits are recorded with negative quantities internally.

---

## 10. API Structure

The API is structured around domain-driven RESTful endpoints:

- `/auth`: Login, registration, and token generation.
- `/places`: CRUD operations for physical storage locations.
- `/products`: CRUD operations for inventory items.
- `/suppliers`: CRUD operations for product vendors.
- `/stock_movements`: Stock transaction management.

Authentication is handled via **JWT Bearer Tokens** using the custom `AuthenticatedUser` extractor.

---

## 11. Code & Contribution Guidelines

- **Rust Best Practices**: All code must pass `cargo fmt` and `cargo clippy` without warnings.
- **Layered Architecture**: Never inject HTTP-specific logic (Axum extractors, status codes) into the `services` layer.
- **DTO Usage**: Always use Data Transfer Objects (DTOs) for payloads (e.g., `CreatePlaceDto`) rather than exposing internal database models.
