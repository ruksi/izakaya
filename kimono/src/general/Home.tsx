import {Counter} from "../Counter";
import Container from "react-bootstrap/Container";

export default function Home() {
    return (
        <Container>
            <h1 className="my-3">Ryokan</h1>
            <p className="my-3">Ryokan is a Japanese-style inn.</p>
            <div className="my-3"><Counter/></div>
        </Container>
    );
}

