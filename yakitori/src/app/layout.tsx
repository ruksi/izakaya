import {MainHeader} from "@/general/MainHeader";
import {config} from "@fortawesome/fontawesome-svg-core";
import type {Metadata} from "next";
import React from "react";
import {Container, Stack} from "react-bootstrap";
import "./globals.scss";

// auto add won't work with Next, so we add the CSS manually in globals.scss
config.autoAddCss = false;

export const metadata: Metadata = {
    title: "Izakaya",
    description: "Rust, React, PaaS, and more!",
    icons: [{url: "/favicon.svg", type: "image/svg+xml"}],
};

export default function MainLayout({children}: React.PropsWithChildren) {
    return (
        <html lang="en" data-bs-theme="dark">
            <body>
                <div className="d-flex flex-column min-vh-100">
                    <header
                        className="bg-body-tertiary border-bottom"
                        style={{minHeight: 64}}
                    >
                        <Container>
                            <Stack
                                direction="horizontal"
                                gap={2}
                                className="p-3"
                            >
                                <MainHeader />
                            </Stack>
                        </Container>
                    </header>
                    <main className="flex-fill d-flex flex-column">
                        {children}
                    </main>
                </div>
            </body>
        </html>
    );
}
