import React, {createContext, FC, ReactElement, ReactNode, useCallback, useContext, useEffect, useMemo, useState} from "react";
import {Alert, Order, Product} from "../types";
import {createOrder, getOrders, getProducts, updateOrder} from "./api";
import {addQuantity, Cart, cartFromOrder, CartSummary, findOpenOrder, itemCount, setQuantity, summarize, toOrderProducts} from "./cart";

interface Shop {
    products: Product[],
    orders: Order[],
    loading: boolean,
    alert?: Alert,
    cart: CartSummary,
    cartCount: number,
    addToCart: (id: string, quantity: number) => Promise<void>,
    updateQuantity: (id: string, quantity: number) => Promise<void>,
    removeFromCart: (id: string) => Promise<void>,
    clearCart: () => Promise<void>,
    checkout: () => Promise<string | undefined>
}

const ShopContext = createContext<Shop | undefined>(undefined);

export const useShop = (): Shop => {
    const shop = useContext(ShopContext);
    if (!shop) {
        throw new Error("useShop must be used within a ShopProvider");
    }
    return shop;
};

export const ShopProvider: FC<{ children: ReactNode }> = ({children}): ReactElement => {
    const [products, setProducts] = useState<Product[]>([]);
    const [orders, setOrders] = useState<Order[]>([]);
    const [loading, setLoading] = useState(true);
    const [alert, setAlert] = useState<Alert | undefined>();

    useEffect(() => {
        Promise.all([getProducts(), getOrders()])
            .then(([loadedProducts, loadedOrders]) => {
                setProducts(loadedProducts);
                setOrders(loadedOrders);
            })
            .catch((error) => {
                console.log("ERROR", error);
                setAlert({variant: "danger", message: "Failed to load data."});
            })
            .finally(() => setLoading(false));
    }, []);

    const openOrder = useMemo(() => findOpenOrder(orders), [orders]);
    const cartItems = useMemo(() => cartFromOrder(openOrder), [openOrder]);
    const cart = useMemo(() => summarize(products, cartItems), [products, cartItems]);

    // Fetch the open order, apply a change to its cart, save it (POST if new, PUT otherwise) and reload.
    const mutate = useCallback(async (change: (cart: Cart) => void, status: "OPEN" | "COMPLETED" = "OPEN"): Promise<Order | undefined> => {
        setAlert(undefined);
        try {
            const current = findOpenOrder(await getOrders());
            const next = cartFromOrder(current);
            change(next);
            let saved: Order | undefined = current;
            if (current) {
                saved = await updateOrder(current.id, status, toOrderProducts(next));
            } else if (next.size > 0) {
                saved = await createOrder(toOrderProducts(next));
            }
            setOrders(await getOrders());
            return saved;
        } catch (error) {
            console.log("ERROR", error);
            setAlert({variant: "danger", message: "Something went wrong, please try again."});
            return undefined;
        }
    }, []);

    const stockOf = useCallback((id: string): number => products.find((p) => p.id === id)?.stock ?? 0, [products]);

    const shop: Shop = {
        products,
        orders,
        loading,
        alert,
        cart,
        cartCount: itemCount(cartItems),
        addToCart: async (id, quantity) => {
            await mutate((c) => addQuantity(c, id, quantity, stockOf(id)));
        },
        updateQuantity: async (id, quantity) => {
            await mutate((c) => setQuantity(c, id, quantity, stockOf(id)));
        },
        removeFromCart: async (id) => {
            await mutate((c) => c.delete(id));
        },
        clearCart: async () => {
            await mutate((c) => c.clear());
        },
        checkout: async () => {
            if (cartItems.size === 0) {
                setAlert({variant: "danger", message: "Your cart is empty."});
                return undefined;
            }
            const order = await mutate(() => undefined, "COMPLETED");
            return order?.id;
        }
    };

    return <ShopContext.Provider value={shop}>{children}</ShopContext.Provider>;
};
