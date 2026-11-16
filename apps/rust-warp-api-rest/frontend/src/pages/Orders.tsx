import React, {FC, ReactElement} from "react";
import {Link} from "react-router-dom";
import {StatusBadge} from "../fragments";
import {useShop} from "../state/ShopContext";

export const Orders: FC = (): ReactElement => {
    const {orders} = useShop();
    const previous = orders.filter((order) => order.status !== "OPEN");

    return (
        <>
            <div className="row">
                <div className="col">
                    <h2 className="emphasized-text">Orders</h2>
                </div>
            </div>
            {previous.length === 0 ? (
                <div className="row mt-4">
                    <div className="col">
                        <p className="text-muted">You have no previous orders.</p>
                    </div>
                </div>
            ) : (
                <div className="row row-cols-1 g-4 mt-1">
                    {previous.map((order) => (
                        <div className="col" key={order.id}>
                            <div className="card emphasized-card order-card h-100">
                                <div className="card-body d-flex flex-column">
                                    <div className="d-flex justify-content-between align-items-start mb-2">
                                        <h5 className="card-title mb-0">Order</h5>
                                        <StatusBadge status={order.status}/>
                                    </div>
                                    <h6 className="card-subtitle mb-3 text-muted text-break">{order.id}</h6>
                                    <p className="card-text flex-grow-1">{order.products.length} product(s)</p>
                                    <Link className="btn btn-primary btn-sm align-self-start" to={`/orders/${order.id}`}>View order</Link>
                                </div>
                            </div>
                        </div>
                    ))}
                </div>
            )}
        </>
    );
};
