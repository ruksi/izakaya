"use client";

import {useVerify} from "@/services/backend";
import React from "react";

export default function AuthenticatedOnly({children}: React.PropsWithChildren) {
    const {isAuthenticated} = useVerify();
    if (isAuthenticated == false) {
        if (typeof window !== "undefined") {
            window.location.replace("/log-in");
        }
        return null;
    }
    return <>{children}</>;
}
