"use client";

import {selectIsAuthenticated} from "@/auth/slice";
import {useAppSelector} from "@/data/hooks";
import React from "react";

export default function AuthenticatedOnly({children}: React.PropsWithChildren) {
    const isAuthenticated = useAppSelector(selectIsAuthenticated);
    if (isAuthenticated == false) {
        if (typeof window !== "undefined") {
            window.location.replace("/log-in");
        }
        return null;
    }
    return <>{children}</>;
}
