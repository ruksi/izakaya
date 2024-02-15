import AuthenticatedOnly from "@/auth/AuthenticatedOnly";
import React from "react";

export default function Layout({children}: React.PropsWithChildren) {
    return <AuthenticatedOnly>{children}</AuthenticatedOnly>;
}
