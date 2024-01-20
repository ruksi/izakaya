import {useRouteError} from "react-router-dom";
import Container from "react-bootstrap/Container";
import MainLayout from "./MainLayout.tsx";

export default function ErrorPage() {
    // https://github.com/remix-run/react-router/discussions/9628#discussioncomment-5555901
    const error: any = useRouteError();

    return (
        <MainLayout>
            <Container id="error-page">
                <div className="p-3">
                    <h1 className="pb-3">
                        <span className="text-body-emphasis">Sorry</span>,
                        an unexpected <span className="text-danger-emphasis">error</span> occurred
                    </h1>
                    <div className="fst-italic text-secondary">
                        {error?.status ? `${error.status} - ` : null}
                        {error.statusText || error.message}
                    </div>
                </div>
            </Container>
        </MainLayout>
    );
}
