import React, {ChangeEvent, FC, ReactElement} from "react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faMinus, faPlus} from "@fortawesome/free-solid-svg-icons";

export interface QuantityPillProps {
    value: number,
    min: number,
    max: number,
    disabled?: boolean,
    onChange: (value: number) => void
}

export const QuantityPill: FC<QuantityPillProps> = (props: QuantityPillProps): ReactElement => {
    const {value, min, max, disabled, onChange} = props;
    const clamp = (n: number): number => Math.max(min, Math.min(max, Number.isNaN(n) ? min : n));

    return (
        <div className="quantity-pill" aria-disabled={disabled}>
            <button type="button" className="quantity-btn" aria-label="Decrease quantity"
                    disabled={disabled} onClick={() => onChange(clamp(value - 1))}>
                <FontAwesomeIcon icon={faMinus}/>
            </button>
            <input type="number" className="quantity-input" aria-label="Quantity"
                   min={min} max={max} value={value} disabled={disabled}
                   onChange={(e: ChangeEvent<HTMLInputElement>) => onChange(clamp(parseInt(e.target.value, 10)))}/>
            <button type="button" className="quantity-btn" aria-label="Increase quantity"
                    disabled={disabled} onClick={() => onChange(clamp(value + 1))}>
                <FontAwesomeIcon icon={faPlus}/>
            </button>
        </div>
    );
};
