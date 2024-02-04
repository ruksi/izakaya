import {Container} from "react-bootstrap";

export default function NotFound() {
    return (
        <Container>
            <div className="py-5 my-5">
                <h1>
                    <span className="text-body-emphasis">Sorry</span>,<br />
                    this page was{" "}
                    <span className="text-danger-emphasis">not found</span>
                </h1>
            </div>
        </Container>
    );
}
