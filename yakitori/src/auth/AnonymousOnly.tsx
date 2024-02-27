"use client";

import {useVerify} from "@/services/backend/auth";
import React from "react";

export default function AnonymousOnly({children}: React.PropsWithChildren) {
    const {isAuthenticated} = useVerify();
    if (isAuthenticated == true) {
        if (typeof window !== "undefined") {
            window.location.replace("/");
        }
        return null;
    }
    return <>{children}</>;
}
