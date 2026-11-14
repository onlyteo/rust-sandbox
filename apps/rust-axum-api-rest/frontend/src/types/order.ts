export type OrderStatus = "OPEN" | "COMPLETED" | "CANCELLED";

export interface OrderProduct {
    id: string,
    quantity: number
}

export interface Order {
    id: string,
    username: string,
    products: OrderProduct[],
    status: OrderStatus,
    timestamp: string
}

export interface CreateOrderRequest {
    products: OrderProduct[]
}

export interface UpdateOrderRequest {
    status: OrderStatus,
    products: OrderProduct[]
}
