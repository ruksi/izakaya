import React from "react";
import ReactDOM from "react-dom/client"
import {Provider} from "react-redux";
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import ErrorPage from "./general/ErrorPage.tsx";
import Home from "./general/Home.tsx";
import Login from "./general/Login.tsx";
import MainLayout from "./general/MainLayout.tsx"
import SignUp from "./general/SignUp.tsx";
import {store} from "./store.ts";
import StyleTester from "./styles/StyleTester.tsx";

import "./styles/main.scss"

const router = createBrowserRouter([
    {
        path: "/",
        element: <MainLayout/>,
        errorElement: <ErrorPage/>,
        children: [
            {
                index: true,
                element: <Home/>,
            },
            {
                path: "login",
                element: <Login/>,
            },
            {
                path: "sign-up",
                element: <SignUp/>,
            },
            {
                path: "styles",
                element: <StyleTester/>,
            },
        ],
    },
]);

ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
        <Provider store={store}>
            <RouterProvider router={router}/>
        </Provider>
    </React.StrictMode>,
)
