## GENERIC ##

# List all apps
apps:
    @ls -1 apps/

# List all libs
libs:
    @ls -1 libs/

# List folders tracked by git
tree:
    tree --gitignore -d

## CARGO ##

# Clean workspace
clean:
    cargo clean

# Run check for workspace or a specific app
check app="":
    cargo check {{ if app == "" { "--workspace" } else { "-p " + app } }}

# Build workspace or a specific app
build app="":
    cargo build {{ if app == "" { "--workspace" } else { "-p " + app } }}

# Run tests for workspace or a specific app
test app="":
    cargo nextest run {{ if app == "" { "--workspace" } else { "-p " + app } }}

# Run clippy linting for workspace or a specific app
clippy app="":
    cargo clippy {{ if app == "" { "--workspace" } else { "-p " + app } }} -- -D warnings

# Run a specific app
run app:
    cargo run -p {{app}}

## DOCKER ##

wiremock-up:
    docker compose -f ./docker/wiremock/docker-compose.yaml up -d

wiremock-stop:
    docker compose -f ./docker/wiremock/docker-compose.yaml stop

wiremock-down:
    docker compose -f ./docker/wiremock/docker-compose.yaml down -v

postgres-up:
    docker compose -f ./docker/postgres/docker-compose.yaml up -d

postgres-stop:
    docker compose -f ./docker/postgres/docker-compose.yaml stop

postgres-down:
    docker compose -f ./docker/postgres/docker-compose.yaml down -v

kafka-up:
    docker compose -f ./docker/kafka/docker-compose.yaml up -d

kafka-stop:
    docker compose -f ./docker/kafka/docker-compose.yaml stop

kafka-down:
    docker compose -f ./docker/kafka/docker-compose.yaml down -v

docker-up: wiremock-up postgres-up kafka-up

docker-stop: wiremock-stop postgres-stop kafka-stop

docker-down: wiremock-down postgres-down kafka-down
