import {useRouteError} from "react-router-dom";

export default function ErrorPage() {
    // https://github.com/remix-run/react-router/discussions/9628#discussioncomment-5555901
    const error: any = useRouteError();

    return (
        <div id="error-page">
            <h1>Oops!</h1>
            <p>Sorry, an unexpected error has occurred.</p>
            <p>
                <i>{error.statusText || error.message}</i>
            </p>
        </div>
    );
}
