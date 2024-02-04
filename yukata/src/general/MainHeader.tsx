"use client";

import {selectIsAuthenticated} from "@/auth/slice";
import {authLogOut} from "@/auth/thunks";
import {useAppDispatch, useAppSelector} from "@/data/hooks";
import tatami from "@/services/tatami";
import {
    faArrowRightToBracket,
    faHouse,
    faPenToSquare,
} from "@fortawesome/free-solid-svg-icons";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import Link from "next/link";
import {useCallback, useEffect} from "react";
import {Button, Placeholder} from "react-bootstrap";

export function MainHeader() {
    const dispatch = useAppDispatch();
    const isAuthenticated = useAppSelector(selectIsAuthenticated);

    // reset all data after log out
    // note sure if this is the right place for this, but 🤷
    // feels like this should be somewhere in Redux side, not React
    useEffect(() => {
        if (isAuthenticated === false) {
            dispatch(tatami.util.resetApiState());
        }
    }, [isAuthenticated, dispatch]);

    const logOut = useCallback(() => {
        dispatch(authLogOut());
    }, [dispatch]);

    return (
        <>
            <div className="d-inline-block me-auto">
                <Link href="/" id="home-link" aria-label="Home" title="Home">
                    <Button
                        variant="outline-secondary"
                        aria-labelledby="home-link"
                        size="sm"
                    >
                        <FontAwesomeIcon icon={faHouse} />
                    </Button>
                </Link>
                {isAuthenticated == true && (
                    <Link href="/dashboard">
                        <Button
                            variant="outline-secondary"
                            size="sm"
                            className="ms-1"
                        >
                            Dashboard
                        </Button>
                    </Link>
                )}
            </div>

            {isAuthenticated == false && (
                <>
                    <Link href="/about">
                        <Button
                            variant="link"
                            size="sm"
                            className="text-decoration-none text-secondary"
                        >
                            About
                        </Button>
                    </Link>
                    <Link href="/log-in">
                        <Button
                            variant="outline-secondary"
                            size="sm"
                            className="text-nowrap"
                        >
                            <FontAwesomeIcon
                                icon={faArrowRightToBracket}
                                className="me-1"
                            />
                            Log in
                        </Button>
                    </Link>
                    <Link href="/sign-up">
                        <Button
                            variant="secondary"
                            size="sm"
                            className="text-nowrap"
                        >
                            <FontAwesomeIcon
                                icon={faPenToSquare}
                                className="me-1"
                            />
                            Sign up
                        </Button>
                    </Link>
                </>
            )}

            {isAuthenticated == true && (
                <>
                    <Link href="/settings">
                        <Button
                            variant="outline-secondary"
                            size="sm"
                            className="ms-1"
                        >
                            <FontAwesomeIcon
                                icon={faPenToSquare}
                                className="me-1"
                            />
                            Settings
                        </Button>
                    </Link>
                    <Button
                        variant="outline-secondary"
                        size="sm"
                        className="ms-1"
                        onClick={logOut}
                    >
                        <FontAwesomeIcon
                            icon={faArrowRightToBracket}
                            className="me-1"
                        />
                        Log out
                    </Button>
                </>
            )}

            {isAuthenticated == null && (
                <>
                    <Button variant="outline-secondary" size="sm">
                        <Placeholder
                            aria-hidden="true"
                            bg="secondary"
                            style={{width: 56}}
                        />
                    </Button>
                    <Button variant="outline-secondary" size="sm">
                        <Placeholder
                            aria-hidden="true"
                            bg="secondary"
                            style={{width: 65}}
                        />
                    </Button>
                </>
            )}
        </>
    );
}
