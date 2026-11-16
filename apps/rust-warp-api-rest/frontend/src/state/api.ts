import {CreateOrderRequest, Order, OrderProduct, OrderStatus, Product, UpdateOrderRequest} from "../types";
import {GET, POST, PUT} from "./client";

export const getProducts = async (): Promise<Product[]> =>
    (await GET<Product[]>("/api/products")).data ?? [];

export const getOrders = async (): Promise<Order[]> =>
    (await GET<Order[]>("/api/orders")).data ?? [];

export const getOrder = async (id: string): Promise<Order | undefined> =>
    (await GET<Order>(`/api/orders/${id}`)).data;

export const createOrder = async (products: OrderProduct[]): Promise<Order | undefined> =>
    (await POST<Order, CreateOrderRequest>("/api/orders", {products})).data;

export const updateOrder = async (id: string, status: OrderStatus, products: OrderProduct[]): Promise<Order | undefined> =>
    (await PUT<Order, UpdateOrderRequest>(`/api/orders/${id}`, {status, products})).data;
