import React, {FC, ReactElement} from "react";
import {Link} from "react-router-dom";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faCartShopping, faScrewdriverWrench, faUser} from "@fortawesome/free-solid-svg-icons";
import {principal} from "../state/principal";
import {useShop} from "../state/ShopContext";

export const Header: FC = (): ReactElement => {
    const {cartCount} = useShop();

    return (
        <header className="header mb-5">
            <nav className="navbar navbar-dark navbar-expand-lg p-4">
                <Link className="navbar-brand mx-4" to="/">
                    <h1>
                        <FontAwesomeIcon className="navbar-logo emphasized-text" icon={faScrewdriverWrench}/>
                        <span className="navbar-title ms-3">Rust Sandbox</span>
                    </h1>
                </Link>
                <div className="ms-auto d-flex gap-2">
                    <Link className="btn btn-outline-light position-relative" to="/cart">
                        <FontAwesomeIcon icon={faCartShopping}/> Cart
                        {cartCount > 0 && (
                            <span className="badge rounded-pill bg-danger cart-badge">{cartCount}</span>
                        )}
                    </Link>
                    <details className="user-menu">
                        <summary className="btn btn-outline-light">
                            <FontAwesomeIcon icon={faUser}/> {principal.name}
                        </summary>
                        <ul className="dropdown-menu dropdown-menu-end show">
                            <li><span className="dropdown-item-text">Signed in as <strong>{principal.username}</strong></span></li>
                            <li><hr className="dropdown-divider"/></li>
                            <li><Link className="dropdown-item" to="/orders">My orders</Link></li>
                        </ul>
                    </details>
                </div>
            </nav>
        </header>
    );
};
