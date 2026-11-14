import React, {FC, ReactElement, useState} from "react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faCartPlus} from "@fortawesome/free-solid-svg-icons";
import {formatEuro} from "../state/cart";
import {useShop} from "../state/ShopContext";
import {Product} from "../types";
import {QuantityPill} from "./QuantityPill";

export const ProductCard: FC<{ product: Product }> = ({product}): ReactElement => {
    const {addToCart} = useShop();
    const [quantity, setQuantity] = useState(1);
    const outOfStock = product.stock === 0;

    const onSubmit = async (e: React.FormEvent<HTMLFormElement>): Promise<void> => {
        e.preventDefault();
        await addToCart(product.id, quantity);
        setQuantity(1);
    };

    return (
        <div className="col">
            <div className="card emphasized-card product-card h-100">
                <div className="card-body d-flex flex-column">
                    <h5 className="card-title">{product.name}</h5>
                    <h6 className="card-subtitle mb-2 text-muted">{product.packing}</h6>
                    <p className="card-text flex-grow-1">{product.description}</p>
                    <div className="d-flex justify-content-between align-items-center mb-3">
                        <span className="product-price">{formatEuro(product.price)}</span>
                        {outOfStock
                            ? <span className="badge bg-secondary">Out of stock</span>
                            : <span className="badge bg-success">{product.stock} in stock</span>}
                    </div>
                    <form onSubmit={onSubmit} className="d-flex align-items-center justify-content-between gap-2">
                        <QuantityPill value={quantity} min={1} max={Math.max(product.stock, 1)}
                                      disabled={outOfStock} onChange={setQuantity}/>
                        <button type="submit" className="btn btn-primary text-nowrap" disabled={outOfStock}>
                            <FontAwesomeIcon icon={faCartPlus}/> Add
                        </button>
                    </form>
                </div>
            </div>
        </div>
    );
};
