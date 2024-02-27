import {createBrowserRouter} from "react-router-dom";
import AnonymousOnly from "./auth/AnonymousOnly.tsx";
import AuthenticatedOnly from "./auth/AuthenticatedOnly.tsx";
import LogIn from "./auth/LogIn.tsx";
import SignUp from "./auth/SignUp.tsx";
import {authVerify} from "./auth/thunks.ts";
import {AboutPage} from "./general/AboutPage.tsx";
import ErrorPage from "./general/ErrorPage.tsx";
import LandingPage from "./general/LandingPage.tsx";
import MainLayout from "./general/MainLayout.tsx";
import {SettingsPage} from "./settings/SettingsPage.tsx";
import {store} from "./store.ts";
import StyleTester from "./styles/StyleTester.tsx";

export const router = createBrowserRouter([
    {
        loader: async () => {
            await store.dispatch(authVerify());
            return null;
        },
        path: "/",
        element: <MainLayout />,
        errorElement: <ErrorPage />,
        children: [
            {
                index: true,
                element: <LandingPage />,
            },
            {
                path: "about",
                element: <AboutPage />,
            },

            {
                element: <AuthenticatedOnly />,
                children: [
                    {
                        path: "settings",
                        element: <SettingsPage />,
                    },
                    {
                        path: "styles",
                        element: <StyleTester />,
                    },
                ],
            },
            {
                element: <AnonymousOnly />,
                children: [
                    {
                        path: "log-in",
                        element: <LogIn />,
                    },
                    {
                        path: "sign-up",
                        element: <SignUp />,
                    },
                ],
            },
        ],
    },
]);
