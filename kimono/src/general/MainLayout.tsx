import viteLogo from "/vite.svg"
import React, {useEffect} from "react";
import Button from "react-bootstrap/Button";
import Container from "react-bootstrap/Container";
import Stack from "react-bootstrap/esm/Stack";
import {useSelector} from "react-redux";
import {Link, Outlet} from "react-router-dom";
import reactLogo from "../assets/react.svg"
import {selectIsAuthenticated} from "../auth/slice.ts";
import {authLogOut} from "../auth/thunks.ts";
import tatami from "../services/tatami.ts";
import {store} from "../store.ts";

export default function MainLayout({children}: { children?: React.ReactNode }) {

    const logOut = () => {
        store.dispatch(authLogOut());
    }

    const isAuthenticated = useSelector(selectIsAuthenticated);

    // reset all data after log out
    // note sure if this is the right place for this, but 🤷
    // feels like this should be somewhere in Redux side, not React
    useEffect(() => {
        if (!isAuthenticated) {
            store.dispatch(tatami.util.resetApiState());
        }
    }, [isAuthenticated]);

    return (
        <div className="d-flex flex-column min-vh-100">

            <header className="bg-body-tertiary">
                <Container>
                    <Stack direction="horizontal" gap={2} className="p-3">
                        <Link to={`/`} className="me-auto">Home</Link>
                        {
                            isAuthenticated &&
                            <>
                                <Link to={`/settings`}>Settings</Link>
                                <Button variant="link" onClick={logOut}>Log Out</Button>
                            </>
                        }
                        {
                            !isAuthenticated &&
                            <>
                                <Link to={`/log-in`}>Log In</Link>
                                <Link to={`/sign-up`}>Sign Up</Link>
                            </>
                        }
                    </Stack>
                </Container>
            </header>

            <main className="flex-grow-1">
                {children ? children : null}
                <Outlet/>
            </main>

            <footer className="bg-body-tertiary text-center">
                <Container className="p-3">
                    <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
                        <img src={viteLogo} className="logo" alt="Vite logo"/>
                    </a>
                    <a href="https://react.dev" target="_blank" rel="noreferrer">
                        <img src={reactLogo} className="logo react" alt="React logo"/>
                    </a>
                </Container>
            </footer>

        </div>
    )
}
