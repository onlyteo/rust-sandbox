import React, {FC, ReactElement, useEffect, useState} from "react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faFloppyDisk, faTrash} from "@fortawesome/free-solid-svg-icons";
import {CartLine, formatEuro} from "../state/cart";
import {useShop} from "../state/ShopContext";
import {QuantityPill} from "./QuantityPill";

export const CartLineCard: FC<{ line: CartLine }> = ({line}): ReactElement => {
    const {updateQuantity, removeFromCart} = useShop();
    const [quantity, setQuantity] = useState(line.quantity);

    useEffect(() => setQuantity(line.quantity), [line.quantity]);

    return (
        <div className="col">
            <div className="card emphasized-card product-card">
                <div className="card-body">
                    <div className="d-flex justify-content-between align-items-start">
                        <div>
                            <h5 className="card-title">{line.name}</h5>
                            <h6 className="card-subtitle mb-2 text-muted">{line.packing}</h6>
                        </div>
                        <div className="d-flex gap-2">
                            {quantity !== line.quantity && (
                                <button type="button" className="btn btn-sm btn-primary"
                                        onClick={() => updateQuantity(line.id, quantity)}>
                                    <FontAwesomeIcon icon={faFloppyDisk}/> Update
                                </button>
                            )}
                            <button type="button" className="btn btn-sm btn-outline-danger" title="Remove"
                                    onClick={() => removeFromCart(line.id)}>
                                <FontAwesomeIcon icon={faTrash}/>
                            </button>
                        </div>
                    </div>
                    <hr/>
                    <div className="row text-center align-items-center">
                        <div className="col-4">
                            <div className="text-muted small text-uppercase">Quantity</div>
                            <QuantityPill value={quantity} min={0} max={line.stock} onChange={setQuantity}/>
                        </div>
                        <div className="col-4">
                            <div className="text-muted small text-uppercase">Item price</div>
                            <div className="fw-bold">{formatEuro(line.unitPrice)}</div>
                        </div>
                        <div className="col-4">
                            <div className="text-muted small text-uppercase">Total price</div>
                            <div className="fw-bold">{formatEuro(line.lineTotal)}</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};
