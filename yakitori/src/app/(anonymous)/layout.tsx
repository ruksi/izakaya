import AnonymousOnly from "@/auth/AnonymousOnly";
import React from "react";

export default function StoredLayout({children}: React.PropsWithChildren) {
    return <AnonymousOnly>{children}</AnonymousOnly>;
}
