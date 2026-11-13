# Tests

## cURL

### Backend

#### GET orders
```shell
curl -u skywalker:T4t01ne -X GET http://localhost:8081/api/orders | jq
```

#### POST order
```shell
curl -u skywalker:T4t01ne -H "Content-Type: application/json" -X POST -d '{"products":[{"id":"d2ac5e0c-83b0-4934-a975-a2134fc1e767","quantity":3}]}' http://localhost:8081/api/orders | jq
```
