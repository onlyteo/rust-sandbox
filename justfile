docker-wiremock-up:
    docker compose -f ./docker/wiremock/docker-compose.yaml up -d

docker-wiremock-stop:
    docker compose -f ./docker/wiremock/docker-compose.yaml stop

docker-wiremock-down:
    docker compose -f ./docker/wiremock/docker-compose.yaml down

docker-postgres-up:
    docker compose -f ./docker/postgres/docker-compose.yaml up -d

docker-postgres-stop:
    docker compose -f ./docker/postgres/docker-compose.yaml stop

docker-postgres-down:
    docker compose -f ./docker/postgres/docker-compose.yaml down

docker-kafka-up:
    docker compose -f ./docker/kafka/docker-compose.yaml up -d

docker-kafka-stop:
    docker compose -f ./docker/kafka/docker-compose.yaml stop

docker-kafka-down:
    docker compose -f ./docker/kafka/docker-compose.yaml down

docker-up: docker-wiremock-up docker-postgres-up docker-kafka-up

docker-stop: docker-wiremock-stop docker-postgres-stop docker-kafka-stop

docker-down: docker-wiremock-down docker-postgres-down docker-kafka-down
