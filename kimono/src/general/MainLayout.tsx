import viteLogo from "/vite.svg"
import React from "react";
import Button from "react-bootstrap/Button";
import Container from "react-bootstrap/Container";
import Stack from "react-bootstrap/esm/Stack";
import {useSelector} from "react-redux";
import {Link, Outlet} from "react-router-dom";
import reactLogo from "../assets/react.svg"
import {selectIsAuthenticated} from "../auth/slice.ts";
import {authLogOut, authVerify} from "../auth/thunks.ts";
import {store} from "../store.ts";

export default function MainLayout({children}: { children: React.ReactNode }) {

    const verify = () => {
        store.dispatch(authVerify());
    }
    const logOut = () => {
        store.dispatch(authLogOut());
    }

    const isAuthenticated = useSelector(selectIsAuthenticated);

    return (
        <div className="d-flex flex-column min-vh-100">

            <header className="bg-body-tertiary">
                <Container>
                    <Stack direction="horizontal" gap={2} className="p-3">
                        <Link to={`/`}>Home</Link>
                        <Button onClick={verify} className="ms-auto">Verify</Button>
                        {
                            isAuthenticated &&
                            <>
                                <Button onClick={logOut}>Log Out</Button>
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
