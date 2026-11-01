# Rust Axum CRUD SQLx

A CRUD example for an Ordering system (Products + Orders), backed by Postgres via `sqlx`.

- `backend` — Axum REST API (port 8081), persists to Postgres, runs its own migrations at startup.
- `frontend` — Axum + Askama server-rendered UI (port 8080), calls the backend directly.

## Running locally

1. Start Postgres: `just docker-postgres-up`
2. Copy `backend/.env.example` to `backend/.env` (defaults already match the compose file).
3. Run the backend — migrations and seed data are applied automatically at startup:
   ```bash
   cargo run --manifest-path=./apps/rust-axum-crud-sqlx/backend/Cargo.toml
   ```
4. Run the frontend:
   ```bash
   cargo run --manifest-path=./apps/rust-axum-crud-sqlx/frontend/Cargo.toml
   ```
5. Browse `http://localhost:8080` — pick product quantities to create an order, then view/update it under "Orders".
