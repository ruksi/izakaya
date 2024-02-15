import AnonymousOnly from "@/auth/AnonymousOnly";
import React from "react";

export default function Layout({children}: React.PropsWithChildren) {
    return <AnonymousOnly>{children}</AnonymousOnly>;
}
