import React, {FC, ReactElement} from "react";
import {Link} from "react-router-dom";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faCartShopping} from "@fortawesome/free-solid-svg-icons";
import {AlertMessage, ProductCard} from "../fragments";
import {formatEuro} from "../state/cart";
import {useShop} from "../state/ShopContext";

export const Home: FC = (): ReactElement => {
    const {products, cart, alert} = useShop();

    return (
        <>
            <div className="row">
                <div className="col">
                    <h2 className="emphasized-text">Welcome to this Rust example!</h2>
                    <p>This example shows a React frontend and Rust/Warp REST API</p>
                </div>
            </div>
            <AlertMessage alert={alert}/>
            <div className="row mt-4">
                <div className="col">
                    <div className="card emphasized-card cart-summary">
                        <div className="card-body">
                            <div className="d-flex justify-content-between align-items-center">
                                <h5 className="card-title mb-0"><FontAwesomeIcon icon={faCartShopping}/> Your cart</h5>
                                <Link className="btn btn-primary btn-sm" to="/cart">View cart</Link>
                            </div>
                            {cart.lines.length === 0 ? (
                                <p className="text-muted mb-0 mt-3">Your cart is empty.</p>
                            ) : (
                                <>
                                    <ul className="list-unstyled mt-3 mb-2">
                                        {cart.lines.map((line) => (
                                            <li key={line.id} className="d-flex justify-content-between">
                                                <span>{line.quantity} &times; {line.name}</span>
                                                <span>{formatEuro(line.lineTotal)}</span>
                                            </li>
                                        ))}
                                    </ul>
                                    <hr/>
                                    <div className="d-flex justify-content-between fw-bold">
                                        <span>Total ({cart.itemCount} items)</span>
                                        <span>{formatEuro(cart.total)}</span>
                                    </div>
                                </>
                            )}
                        </div>
                    </div>
                </div>
            </div>
            <div className="row row-cols-1 row-cols-md-2 row-cols-lg-3 g-4 mt-1">
                {products.map((product) => <ProductCard key={product.id} product={product}/>)}
            </div>
        </>
    );
};
