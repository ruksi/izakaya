"use client";

import {useCurrentUser} from "@/services/backend/user";
import {Placeholder} from "react-bootstrap";

export function UserSection() {
    const {user} = useCurrentUser();
    return (
        <div>
            <label className="text-body-emphasis me-1">Username:</label>
            {user ? (
                user.username
            ) : (
                <Placeholder
                    aria-hidden="true"
                    bg="secondary"
                    style={{width: 60}}
                />
            )}
        </div>
    );
}
