import React, {FC, ReactElement, useEffect, useState} from "react";
import {useParams} from "react-router-dom";
import {StatusBadge} from "../fragments";
import {getOrder} from "../state/api";
import {formatEuro} from "../state/cart";
import {useShop} from "../state/ShopContext";
import {Order} from "../types";
import {NotFound} from "./NotFound";

export const OrderDetail: FC = (): ReactElement => {
    const {id} = useParams();
    const {products} = useShop();
    const [order, setOrder] = useState<Order | undefined>();
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        if (!id) {
            setOrder(undefined);
            setLoading(false);
            return;
        }
        setLoading(true);
        getOrder(id)
            .then(setOrder)
            .catch(() => setOrder(undefined))
            .finally(() => setLoading(false));
    }, [id]);

    if (loading) {
        return <></>;
    }
    // Open orders are the cart, so they have no detail page
    if (!order || order.status === "OPEN") {
        return <NotFound/>;
    }

    return (
        <>
            <div className="row">
                <div className="col">
                    <h2 className="emphasized-text">Order</h2>
                    <h4 className="text-muted text-break">{order.id}</h4>
                    <StatusBadge status={order.status}/>
                </div>
            </div>
            <div className="row row-cols-1 g-4 mt-1">
                {order.products.map((line) => {
                    const product = products.find((p) => p.id === line.id);
                    return (
                        <div className="col" key={line.id}>
                            <div className="card emphasized-card product-card">
                                <div className="card-body">
                                    <h5 className="card-title">{product?.name ?? line.id}</h5>
                                    <h6 className="card-subtitle mb-2 text-muted">{product?.packing}</h6>
                                    <p className="card-text">{product?.description}</p>
                                    <hr/>
                                    <div className="row text-center">
                                        <div className="col-4">
                                            <div className="text-muted small text-uppercase">Quantity</div>
                                            <div className="fw-bold">{line.quantity}</div>
                                        </div>
                                        <div className="col-4">
                                            <div className="text-muted small text-uppercase">Item price</div>
                                            <div className="fw-bold">{product && formatEuro(product.price)}</div>
                                        </div>
                                        <div className="col-4">
                                            <div className="text-muted small text-uppercase">Total price</div>
                                            <div className="fw-bold">{product && formatEuro(product.price * line.quantity)}</div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    );
                })}
            </div>
        </>
    );
};
