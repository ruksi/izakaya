import React from "react";
import Spinner from "react-bootstrap/Spinner";

const ButtonSpinnerIf = ({isLoading, children}: { isLoading: boolean, children: React.ReactNode }) => {
    return (
        isLoading
            ? (
                <Spinner as="span" animation="border" size="sm" role="status" aria-hidden="true">
                    <span className="visually-hidden">Loading...</span>
                </Spinner>
            ) : children
    );
};


export default ButtonSpinnerIf;
