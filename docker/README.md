# Docker local environment

## Start containers
```bash
docker compose -f ./docker/wiremock/docker-compose.yaml up -d
```
```bash
docker compose -f ./docker/postgres/docker-compose.yaml up -d
```
```bash
docker compose -f ./docker/kafka/docker-compose.yaml up -d
```

## Stop containers
```bash
docker compose -f ./docker/wiremock/docker-compose.yaml stop
```
```bash
docker compose -f ./docker/postgres/docker-compose.yaml stop
```
```bash
docker compose -f ./docker/kafka/docker-compose.yaml stop
```

## Delete containers
```bash
docker compose -f ./docker/wiremock/docker-compose.yaml down -v
```
```bash
docker compose -f ./docker/postgres/docker-compose.yaml down -v
```
```bash
docker compose -f ./docker/kafka/docker-compose.yaml down -v
```
