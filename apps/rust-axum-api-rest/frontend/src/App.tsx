import React, {FC, ReactElement} from "react";
import {createBrowserRouter, Outlet, RouterProvider} from "react-router-dom";
import {Footer, Header} from "./fragments";
import {Cart, Home, NotFound, OrderDetail, Orders} from "./pages";
import {ShopProvider} from "./state/ShopContext";

const Layout: FC = (): ReactElement => (
    <ShopProvider>
        <Header/>
        <main className="container" role="main">
            <div className="description-box px-3 py-5 rounded-3">
                <Outlet/>
            </div>
        </main>
        <Footer/>
    </ShopProvider>
);

const router = createBrowserRouter([
    {
        element: <Layout/>,
        children: [
            {path: "/", element: <Home/>},
            {path: "/cart", element: <Cart/>},
            {path: "/orders", element: <Orders/>},
            {path: "/orders/:id", element: <OrderDetail/>},
            {path: "*", element: <NotFound/>}
        ]
    }
]);

const App: FC = (): ReactElement => <RouterProvider router={router}/>;

export default App;
