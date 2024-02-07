"use client";

import {selectIsAuthenticated} from "@/auth/slice";
import {useAppSelector} from "@/data/hooks";
import React from "react";

export default function AnonymousOnly({children}: React.PropsWithChildren) {
    const isAuthenticated = useAppSelector(selectIsAuthenticated);
    if (isAuthenticated == true) {
        if (typeof window !== "undefined") {
            window.location.replace("/dashboard");
        }
        return null;
    }
    return <>{children}</>;
}
