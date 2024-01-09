import {Counter} from "../Counter";

export default function Home() {
    return (
        <div className="p-3">
            <h1 className="my-3">Ryokan</h1>
            <p className="my-3">Ryokan is a Japanese-style inn.</p>
            <div className="my-3"><Counter/></div>
        </div>
    );
}

