# Change logic of examples
This file describes key points for code change of the examples in this monorepo.
The reference implementation is `apps/rust-axum-crud-sqlx`. Other examples should follow the same domain model,
API contract and behaviour described here.

## Overview
The logic of the example applications changes from the simple Greeting flow to an Ordering system.
Users browse products, collect them in a cart, and check out to complete an order.

Reference architecture (`rust-axum-crud-sqlx`):
```
Axum + Askama frontend (port 8080) → Axum REST backend (port 8081) → Postgres (sqlx)
```
Crates: `rust-axum-crud-sqlx-backend` and `rust-axum-crud-sqlx-frontend`. Both use the shared libs in `libs/`
(`server`, `yaml_config`, `observability`; the backend also uses `error_handling` and `auth_handling` with the `basic_auth` feature).

## Domain objects

### Product domain
The Product domain contains details on the available products.

JSON representation:
```json
{
    "id": "6821a470-071b-4cf4-b993-43cc8b2a8827",
    "name": "Roasted Coffee Beans",
    "description": "Medium roasted coffee beans",
    "packing": "1kg bag",
    "price": 10,
    "stock": 100
}
```
- `price` is a whole-euro integer (`i32`), not a string. The frontend formats it for display (`10` -> `€10`).
- `stock` is an `i32`. Both `price` and `stock` must be 0 or more.

### Order domain
The Order domain contains the products selected by the current user. An order with status `OPEN` is the user's cart.

JSON representation:
```json
{
    "id": "6821a470-071b-4cf4-b993-43cc8b2a8827",
    "username": "skywalker",
    "products": [
        {
            "id": "6821a470-071b-4cf4-b993-43cc8b2a8827",
            "quantity": 2
        }
    ],
    "status": "OPEN",
    "timestamp": "2026-11-13T10:15:30.123456Z"
}
```
- `username` is the owner, taken from the authenticated principal.
- `timestamp` is the creation time in UTC.
- `status` is one of `OPEN`, `COMPLETED` or `CANCELLED`.
- Product `quantity` is an `i32` greater than 0.

Request bodies:
- Create order: `{"products": [{"id", "quantity"}, ...]}`
- Update order: `{"status": "...", "products": [{"id", "quantity"}, ...]}`

## Business logic

### Backend
The backend is an Axum REST API that owns the data store and all business rules.

#### Data store
Postgres via `sqlx` (`PgPool`; `url` and `max_connections` in `config/{local,docker}/database_config.yaml`).
Migrations in `backend/migrations/` run automatically at startup (`sqlx::migrate!`):
- `0001_initial_model.sql` creates `products`, `orders` and `order_items`.
  `order_items` has primary key `(order_id, product_id)`, `ON DELETE CASCADE` on the order, and `CHECK (quantity > 0)`.
- `0002_seed_data.sql` seeds six coffee products (`ON CONFLICT (id) DO NOTHING`).

Local Postgres is started with `just postgres-up` (docker compose in `docker/postgres/`).

#### API
All endpoints are protected by `basic_auth_layer()` from `auth_handling`. The authenticated username scopes every order
query, so a user only sees and changes their own orders.

| Method | Path               | Description                                           | Success |
|--------|--------------------|-------------------------------------------------------|---------|
| GET    | `/api/products`    | All products, ordered by name                         | 200     |
| GET    | `/api/orders`      | All of the user's orders, ordered by timestamp        | 200     |
| GET    | `/api/orders/{id}` | One order                                             | 200     |
| POST   | `/api/orders`      | Create an order from `{"products": [...]}`            | 201     |
| PUT    | `/api/orders/{id}` | Replace status and products: `{"status", "products"}` | 200     |

#### Business rules
- A user can have only one `OPEN` order. Creating a second returns `409 Conflict` (`urn:app:order-already-exists`).
- Update syncs the order items: items not in the request are deleted, the rest are upserted, then the status is updated.
- An unknown order id, or one owned by another user, returns `404` (`urn:app:order-not-found`).
- Stock is **not** decremented on checkout. Only the frontend caps cart quantities at current stock.

#### Code structure (`backend/src/`)
- `route/` — `api.rs` (handlers), `resources.rs` (static files), `mod.rs` (composition + `error_handling_layer`).
- `service/` — `OrderService`, `ProductService`. Each method opens a transaction, calls repositories, and commits.
- `repository/` — plain SQL functions taking `&mut Transaction`: `order.rs`, `order_item.rs`, `product.rs`.
  Orders are fetched with a `LEFT JOIN` on `order_items` and folded into `Order` structs by `map_orders`.
- `model/` — `Product`, `Order`, `OrderProduct`, `OrderStatus`, `CreateOrderRequest`, `UpdateOrderRequest`.
- `state.rs` — `AppState` holding the services. `config.rs` — YAML config loading.

#### Error handling
`DatabaseError` (`thiserror`) wraps `sqlx` failures. `AppError` wraps it and implements `IntoResponse`, producing
RFC 7807 `ProblemDetails` bodies. Unexpected database errors are logged and returned as a generic 500.

#### Tests
Integration tests in `backend/tests/` (`product_api.rs`, `order_api.rs`) use `#[sqlx::test]`, which creates a database
per test and applies the migrations. They require a running Postgres. `tests/resources/backend.http` has manual requests.

### Frontend API
The reference example has no frontend API application. If an example does have one, it is a simple layer between a
JavaScript frontend and the backend. It must expose the same endpoints as the backend API above (same paths, methods,
bodies and status codes), and proxy every call to the backend, forwarding the Basic auth credentials.

### Frontend
The frontend is either a Rust application that communicates with the backend directly, or a JavaScript application that
communicates with the frontend API. Layout and styling must be the same regardless of framework.

The reference frontend is server-rendered with Axum + Askama (templates in `frontend/templates/`: `base.html`,
`index.html`, `cart.html`, `orders.html`, `order_detail.html`) and uses `reqwest` to call the backend (`BACKEND_URL` /
`app_config.yaml`).

#### Authentication
There is no login. A `BootstrapPrincipal` (name, username, password) is read from `app_config.yaml` at startup and sent
as Basic auth on every backend call. The name is shown in the page header.

#### Pages and routes
| Route                 | Description                                                       |
|-----------------------|-------------------------------------------------------------------|
| `GET /`               | Product list with add-to-cart forms, cart summary                 |
| `GET /cart`           | Cart view                                                         |
| `POST /cart/add`      | Add quantity to cart (capped at stock)                            |
| `POST /cart/update`   | Set quantity (capped at stock; 0 removes)                         |
| `POST /cart/remove`   | Remove a product                                                  |
| `POST /cart/clear`    | Empty the cart                                                    |
| `POST /cart/checkout` | Update the open order with status `COMPLETED`, redirect to detail |
| `GET /orders`         | Completed and cancelled orders (the open order is hidden)         |
| `GET /orders/{id}`    | Order detail with product lines and totals (404 for open orders)  |

Mutations use HTML forms with POST-redirect-GET, without JavaScript. A JavaScript frontend should reproduce the same
pages and behaviour.

#### The cart is the open order
The frontend keeps no session state. The cart (product id -> quantity) is derived from the user's `OPEN` order on every
request and saved back with `save_cart`: POST to create the order if none exists, PUT otherwise. The cart logic handles
add/set/remove and stock capping, and computes line and total prices. Unit tests for `Cart`, `CartSummary` and
`format_euro` live in `frontend/src/model/cart.rs`.
