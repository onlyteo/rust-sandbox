import React, {FC, ReactElement} from "react";
import {Link, useNavigate} from "react-router-dom";
import {AlertMessage, CartLineCard} from "../fragments";
import {formatEuro} from "../state/cart";
import {useShop} from "../state/ShopContext";

export const Cart: FC = (): ReactElement => {
    const {cart, alert, clearCart, checkout} = useShop();
    const navigate = useNavigate();

    const onCheckout = async (): Promise<void> => {
        const id = await checkout();
        if (id) {
            navigate(`/orders/${id}`);
        }
    };

    return (
        <>
            <div className="row">
                <div className="col">
                    <h2 className="emphasized-text">Your cart</h2>
                </div>
            </div>
            <AlertMessage alert={alert}/>
            {cart.lines.length === 0 ? (
                <div className="row mt-4">
                    <div className="col">
                        <p>Your cart is empty.</p>
                        <Link className="btn btn-primary" to="/">Continue shopping</Link>
                    </div>
                </div>
            ) : (
                <>
                    <div className="row row-cols-1 g-4 mt-1">
                        {cart.lines.map((line) => <CartLineCard key={line.id} line={line}/>)}
                    </div>
                    <div className="row mt-4">
                        <div className="col">
                            <div className="card emphasized-card cart-summary">
                                <div className="card-body d-flex justify-content-between fw-bold">
                                    <span>Total ({cart.itemCount} items)</span>
                                    <span>{formatEuro(cart.total)}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div className="row mt-4">
                        <div className="col d-flex flex-wrap gap-2 justify-content-between">
                            <Link className="btn btn-outline-secondary" to="/">Continue shopping</Link>
                            <div className="d-flex gap-2">
                                <button type="button" className="btn btn-outline-danger" onClick={clearCart}>Clear cart</button>
                                <button type="button" className="btn btn-primary" onClick={onCheckout}>Place order</button>
                            </div>
                        </div>
                    </div>
                </>
            )}
        </>
    );
};
