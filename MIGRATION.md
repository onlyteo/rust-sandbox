# Change logic of examples
This file describes key points for code change of the examples in this monorepo.

## Overview
I want to change the logic of the mentioned example applications from the current simple Greeting flow to an Ordering system.

## Domain objects

### Product domain
We will need an Product domain that contains details on the available products.

JSON representation of the Product domain:
```json
{
    "id": "6821a470-071b-4cf4-b993-43cc8b2a8827",
    "name": "Roasted Coffee Beans",
    "description": "Medium roasted coffee beans",
    "packing": "1kg bag",
    "price": "€10",
    "stock": 100
}
```

### Order domain
We also need an Order domain that contain products that have been selected by the current user.

JSON representation of the Order domain:
```json
{
    "id": "6821a470-071b-4cf4-b993-43cc8b2a8827",
    "products": [
        {
            "id": "6821a470-071b-4cf4-b993-43cc8b2a8827",
            "quantity": 2
        }
    ],
    "status": "OPEN"
}
```

## Business logic

### Backend
The backend application needs to have a data store for both products and orders. Either in memory, or in a database if used.
There must also be an API with an endpoint to fetch all available products, and endpoints to read and write orders. Orders can be created, updated and fetched.

### Frontend API
If the example has a frontend API application it is a simple layer between a JavaScript frontend and the backend.
It needs to have the same API endpoints as the backend application and proxy any calls to the backend.

### Frontend
The frontend application is either a Rust based application that communicates with the backend directly, or it is an JavaScript application that communicates with the frontend API.
The layout and styling of the frontend should be the same no matter which framework the frontend is built with.
