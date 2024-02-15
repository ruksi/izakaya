"use client";

import {useCurrentUser} from "@/services/backend";
import {Placeholder} from "react-bootstrap";

export function UserControl() {
    const {user} = useCurrentUser();
    return (
        <>
            <h4 className="mt-3">Profile</h4>
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
        </>
    );
}
