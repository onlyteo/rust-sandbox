import {Order, OrderProduct, Product} from "../types";

// The cart is the user's OPEN order: product id -> quantity.
export type Cart = Map<string, number>;

export interface CartLine {
    id: string,
    name: string,
    packing: string,
    unitPrice: number,
    quantity: number,
    stock: number,
    lineTotal: number
}

export interface CartSummary {
    lines: CartLine[],
    itemCount: number,
    total: number
}

export const formatEuro = (amount: number): string => `€${amount}`;

export const findOpenOrder = (orders: Order[]): Order | undefined =>
    orders.find((order) => order.status === "OPEN");

export const cartFromOrder = (order?: Order): Cart =>
    new Map((order?.products ?? [])
        .filter((product) => product.quantity > 0)
        .map((product) => [product.id, product.quantity]));

export const setQuantity = (cart: Cart, id: string, quantity: number, stock: number): void => {
    const capped = Math.min(quantity, stock);
    if (capped > 0) {
        cart.set(id, capped);
    } else {
        cart.delete(id);
    }
};

export const addQuantity = (cart: Cart, id: string, quantity: number, stock: number): void =>
    setQuantity(cart, id, (cart.get(id) ?? 0) + quantity, stock);

export const itemCount = (cart: Cart): number =>
    Array.from(cart.values()).reduce((sum, quantity) => sum + quantity, 0);

export const toOrderProducts = (cart: Cart): OrderProduct[] =>
    Array.from(cart.entries()).map(([id, quantity]) => ({id, quantity}));

export const summarize = (products: Product[], cart: Cart): CartSummary => {
    const lines: CartLine[] = [];
    cart.forEach((quantity, id) => {
        const product = products.find((p) => p.id === id);
        if (product) {
            lines.push({
                id,
                name: product.name,
                packing: product.packing,
                unitPrice: product.price,
                quantity,
                stock: product.stock,
                lineTotal: product.price * quantity
            });
        }
    });
    return {
        lines,
        itemCount: lines.reduce((sum, line) => sum + line.quantity, 0),
        total: lines.reduce((sum, line) => sum + line.lineTotal, 0)
    };
};
