import React, {useEffect} from "react";
import Button from "react-bootstrap/Button";
import Container from "react-bootstrap/Container";
import Stack from "react-bootstrap/esm/Stack";
import {useSelector} from "react-redux";
import {Outlet, useNavigate} from "react-router-dom";
import {selectIsAuthenticated} from "../auth/slice.ts";
import {authLogOut} from "../auth/thunks.ts";
import tatami from "../services/tatami.ts";
import {store} from "../store.ts";

export default function MainLayout({children}: { children?: React.ReactNode }) {

    const isAuthenticated = useSelector(selectIsAuthenticated);
    // reset all data after log out
    // note sure if this is the right place for this, but 🤷
    // feels like this should be somewhere in Redux side, not React
    useEffect(() => {
        if (!isAuthenticated) {
            store.dispatch(tatami.util.resetApiState());
        }
    }, [isAuthenticated]);

    const navigate = useNavigate();

    const logOut = () => {
        store.dispatch(authLogOut());
    }

    return (
        <div className="d-flex flex-column min-vh-100">

            <header className="bg-body-tertiary">
                <Container>
                    <Stack direction="horizontal" gap={2} className="p-3">
                        <Button variant="outline-secondary" onClick={() => navigate("/")} className="me-auto">
                            <em>Ryokan</em>
                        </Button>
                        {
                            isAuthenticated &&
                            <>
                                <Button variant="secondary" onClick={() => navigate("/settings")}>
                                    Settings
                                </Button>
                                <Button variant="outline-secondary" onClick={logOut}>
                                    Log Out
                                </Button>
                            </>
                        }
                        {
                            !isAuthenticated &&
                            <>
                                <Button variant="outline-secondary" onClick={() => navigate("/log-in")}>
                                    Log In
                                </Button>
                                <Button variant="secondary" onClick={() => navigate(`/sign-up`)}>
                                    Sign Up
                                </Button>
                            </>
                        }
                    </Stack>
                </Container>
            </header>

            <main className="flex-fill d-flex flex-column">
                {children ? children : null}
                <Outlet/>
            </main>

        </div>
    )
}
