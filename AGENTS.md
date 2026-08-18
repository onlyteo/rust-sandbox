# Project Context

This file provides guidance to AI agents when working with code in this repository.

## Project Overview

Multi-module Cargo sandbox showcasing various Rust patterns. Each `apps/` module is a standalone example
demonstrating a specific architectural concern. Shared infrastructure lives in `libs/`.

**Stack**: Rust · Axum 0.8.x · Warp 0.4.x · Cargo

## Build Commands

**Rust (workspace):**
```bash
cargo build                         # build all workspace members
cargo build --manifest-path=./apps/rust-axum-api-rest/backend/Cargo.toml
cargo build --manifest-path=./apps/rust-axum-api-rest/frontend-api/Cargo.toml
cargo build --manifest-path=./apps/rust-warp-api-rest/backend/Cargo.toml
cargo build --manifest-path=./apps/rust-warp-api-rest/frontend-api/Cargo.toml
cargo clippy                        # lint all
cargo test                          # run all tests
```

**React frontends** (run inside an app's `frontend/` directory):
```bash
npm install
npm start       # webpack dev server
npm run build   # production build
npm run lint
npm run lint:fix
```

## Architecture

Each example follows a three-tier pattern:

```
React Frontend (port 3000) → Rust Frontend API (port 8080) → Rust Backend (port 8081)
```

- **React Frontend** — TypeScript/React UI, served via webpack dev server. Sends requests to the Frontend API.
- **Rust Frontend API** — Axum or Warp server that proxies/adapts requests from the browser to the backend. Uses `reqwest` (Axum example) or `hyper` directly (Warp example) as the HTTP client.
- **Rust Backend** — Axum or Warp server that implements business logic. Serves both the REST API and static assets (the compiled React app) from its `resources/static/` directory.

## Workspace Layout

```
Cargo.toml                          # workspace root (resolver = "3", edition = 2024)
apps/
  rust-axum-api-rest/
    backend/                        # crate: rust-axum-rest-api-backend
    frontend-api/                   # crate: rust-axum-rest-api-frontend-api
    frontend/                       # React/TypeScript
  rust-warp-api-rest/
    backend/                        # crate: rust-warp-rest-api-backend
    frontend-api/                   # crate: rust-warp-rest-api-frontend-api
    frontend/                       # React/TypeScript
libs/                               # reserved for shared library crates (currently empty)
```

All Rust dependencies are declared in the workspace `Cargo.toml`; individual crate `Cargo.toml` files reference them with `{ workspace = true }`.

## Key Patterns

**Route organisation** — each Rust crate has a `src/route/` module split into:
- `mod.rs` — composes sub-routers into the top-level router
- `api.rs` — JSON REST endpoints (e.g. `POST /api/greetings`)
- `resources.rs` — static file serving

**Error handling** — `thiserror` for typed domain errors, `anyhow` for propagation across call boundaries.

**Data model** — the greeting use case uses `Person` (input) → `Greeting` (output) structs in `src/model/`. Both are `serde`-serializable.

**Static assets** — the backend serves the compiled React build from `resources/static/`. When developing, run the React dev server separately and point it at the frontend-api.
