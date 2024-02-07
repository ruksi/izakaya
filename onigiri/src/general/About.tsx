import Container from "react-bootstrap/Container";
import Stack from "react-bootstrap/Stack";
import svgPurpleHeart from "../assets/purple-heart.svg";
import useTitle from "./useTitle.ts";

export function About() {
    useTitle("About");
    return (
        <Container className="py-3">

            <Stack gap={3}>

                <div>
                    <div>
                        What is this website?&nbsp;
                        <a href="https://github.com/ruksi/izakaya">TL;DR: a Rust+React example project</a>
                    </div>
                    <div>
                        Made with
                        <div className="d-inline-block mx-1" style={{width: "1.5rem", height: "auto"}}>
                            <img src={svgPurpleHeart} alt="love" title="love"/>
                        </div>
                        by <a href="https://ruk.si/">Ruksi</a>
                    </div>
                    <div>
                        License: <a href="https://github.com/ruksi/izakaya/blob/main/LICENSE">MIT</a>
                    </div>
                </div>

                <div>
                    <div className="text-body-emphasis">
                        Multicolor emojis by <a href="https://openmoji.org/">OpenMoji</a>
                    </div>
                    <div>
                        License: <a href="https://creativecommons.org/licenses/by-sa/4.0/#">CC BY-SA 4.0</a>
                    </div>
                </div>

                <div>
                    <div className="text-body-emphasis">
                        Monochrome interface icons by <a href="https://fontawesome.com/license/free">Font Awesome</a>
                    </div>
                    <div>
                        License: <a href="https://creativecommons.org/licenses/by-sa/4.0/#">CC BY-SA 4.0</a>
                    </div>
                </div>

            </Stack>

        </Container>
    );
}