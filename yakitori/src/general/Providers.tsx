"use client";

import {authVerify} from "@/auth/thunks";
import {AppStore, makeStore} from "@/data/store";
import React, {useRef} from "react";
import {Provider} from "react-redux";

export default function Providers({children}: React.PropsWithChildren) {
    const storeRef = useRef<AppStore>();
    if (!storeRef.current) {
        storeRef.current = makeStore();
        if (typeof window !== "undefined") {
            // no need to run this when building
            storeRef.current.dispatch(authVerify());
        }
    }
    return <Provider store={storeRef.current}>{children}</Provider>;
}
