CREATE TABLE products
(
    id          UUID PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    description TEXT         NOT NULL,
    packing     VARCHAR(20)  NOT NULL,
    price       VARCHAR(20)  NOT NULL,
    stock       INTEGER      NOT NULL CHECK (stock > -1)
);

CREATE TABLE orders
(
    id        UUID PRIMARY KEY,
    username  VARCHAR(20) NOT NULL,
    status    VARCHAR(10) NOT NULL,
    timestamp TIMESTAMP(6) NOT NULL
);

CREATE TABLE order_items
(
    order_id   UUID    NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    product_id UUID    NOT NULL REFERENCES products (id),
    quantity   INTEGER NOT NULL CHECK (quantity > 0),
    PRIMARY KEY (order_id, product_id)
);
