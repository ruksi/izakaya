"use client";

import React from "react";
import {Spinner} from "react-bootstrap";

export default function SpinnerIf({
    isLoading,
    children,
}: React.PropsWithChildren<{isLoading: boolean}>) {
    return isLoading ? (
        <Spinner
            as="span"
            animation="border"
            size="sm"
            role="status"
            aria-hidden="true"
        >
            <span className="visually-hidden">Loading...</span>
        </Spinner>
    ) : (
        children
    );
}
