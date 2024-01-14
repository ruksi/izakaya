import {createBrowserRouter} from "react-router-dom";
import {authVerify} from "./auth/thunks.ts";
import AnonymousOnly from "./general/AnonymousOnly.tsx";
import AuthenticatedOnly from "./general/AuthenticatedOnly.tsx";
import ErrorPage from "./general/ErrorPage.tsx";
import Home from "./general/Home.tsx";
import LogIn from "./general/LogIn.tsx";
import MainLayout from "./general/MainLayout.tsx";
import SignUp from "./general/SignUp.tsx";
import {store} from "./store.ts";
import StyleTester from "./styles/StyleTester.tsx";

export const router = createBrowserRouter([
    {
        loader: async () => {
            await store.dispatch(authVerify());
            return null;
        },
        path: "/",
        element: <MainLayout/>,
        errorElement: <ErrorPage/>,
        children: [
            {
                index: true,
                element: <Home/>,
            },

            {
                element: <AuthenticatedOnly/>,
                children: [
                    {
                        path: "styles",
                        element: <StyleTester/>,
                    },
                ],
            },
            {
                element: <AnonymousOnly/>,
                children: [
                    {
                        path: "log-in",
                        element: <LogIn/>,
                    },
                    {
                        path: "sign-up",
                        element: <SignUp/>,
                    },
                ],
            },
        ],
    },
]);
