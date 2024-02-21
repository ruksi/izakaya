"use client";

import {useLogOut, useVerify} from "@/services/backend/auth";
import {
    faArrowRightToBracket,
    faGear,
    faHouse,
    faPenToSquare,
    faRightFromBracket,
} from "@fortawesome/free-solid-svg-icons";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import Link from "next/link";
import {Button, Placeholder} from "react-bootstrap";

export function MainHeader() {
    const {isAuthenticated} = useVerify();
    const {logOut} = useLogOut();

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
                {isAuthenticated && (
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

            {isAuthenticated && (
                <>
                    <Link href="/settings">
                        <Button variant="outline-secondary" size="sm">
                            <FontAwesomeIcon icon={faGear} className="me-1" />
                            Settings
                        </Button>
                    </Link>
                    <Button
                        variant="outline-secondary"
                        size="sm"
                        onClick={() => logOut()}
                    >
                        <FontAwesomeIcon
                            icon={faRightFromBracket}
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
