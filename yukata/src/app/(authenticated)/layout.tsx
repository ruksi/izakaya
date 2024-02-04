import AuthenticatedOnly from "@/auth/AuthenticatedOnly";
import React from "react";

export default function StoredLayout({children}: React.PropsWithChildren) {
    return <AuthenticatedOnly>{children}</AuthenticatedOnly>;
}
