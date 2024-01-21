import {createBrowserRouter} from "react-router-dom";
import AnonymousOnly from "./auth/AnonymousOnly.tsx";
import AuthenticatedOnly from "./auth/AuthenticatedOnly.tsx";
import LogIn from "./auth/LogIn.tsx";
import SignUp from "./auth/SignUp.tsx";
import {authVerify} from "./auth/thunks.ts";
import ErrorPage from "./general/ErrorPage.tsx";
import Home from "./general/Home.tsx";
import MainLayout from "./general/MainLayout.tsx";
import {Settings} from "./settings/Settings.tsx";
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
                        path: "settings",
                        element: <Settings/>,
                    },
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
