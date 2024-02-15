"use client";

import {useVerify} from "@/services/backend";
import React from "react";

export default function AnonymousOnly({children}: React.PropsWithChildren) {
    const {isAuthenticated} = useVerify();
    if (isAuthenticated == true) {
        if (typeof window !== "undefined") {
            window.location.replace("/dashboard");
        }
        return null;
    }
    return <>{children}</>;
}
