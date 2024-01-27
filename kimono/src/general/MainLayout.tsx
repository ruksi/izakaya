import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
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

// TODO: move this somewhere else
import {library} from "@fortawesome/fontawesome-svg-core"
import {fas} from "@fortawesome/free-solid-svg-icons"

library.add(fas);

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

            <header className="bg-body-tertiary border-bottom">
                <HeaderWrapper isAuthenticated={isAuthenticated}>
                    <Stack direction="horizontal" gap={2} className="p-3">
                        <Button variant="outline-secondary" size="sm" onClick={() => navigate("/")} className="me-auto">
                            {isAuthenticated
                                ? <>Dashboard</>
                                : <><em>Ryokan?</em></>}
                        </Button>
                        {
                            isAuthenticated &&
                            <>
                                <Button variant="outline-secondary" size="sm" onClick={() => navigate("/settings")}>
                                    <FontAwesomeIcon icon="gear" className="me-1"/>Settings
                                </Button>
                                <Button variant="outline-secondary" size="sm" onClick={logOut}>
                                    <FontAwesomeIcon icon="right-from-bracket" className="me-1"/>Log out
                                </Button>
                            </>
                        }
                        {
                            !isAuthenticated &&
                            <>
                                <Button
                                    variant="link"
                                    size="sm"
                                    onClick={() => navigate("/about")}
                                    className="text-nowrap text-decoration-none text-secondary"
                                >
                                    About
                                </Button>
                                <Button
                                    variant="outline-secondary"
                                    size="sm"
                                    onClick={() => navigate("/log-in")}
                                    className="text-nowrap"
                                >
                                    <FontAwesomeIcon icon="arrow-right-to-bracket" className="me-1"/>Log in
                                </Button>
                                <Button
                                    variant="secondary"
                                    size="sm"
                                    onClick={() => navigate(`/sign-up`)}
                                    className="text-nowrap"
                                >
                                    <FontAwesomeIcon icon="pen-to-square" className="me-1"/>Sign up
                                </Button>
                            </>
                        }
                    </Stack>
                </HeaderWrapper>
            </header>

            <main className="flex-fill d-flex flex-column">
                {children ? children : null}
                <Outlet/>
            </main>

        </div>
    )
}

function HeaderWrapper({isAuthenticated, children}: { isAuthenticated: boolean, children?: React.ReactNode }) {
    return isAuthenticated
        ? <>{children}</>
        : <Container>{children}</Container>;
}
