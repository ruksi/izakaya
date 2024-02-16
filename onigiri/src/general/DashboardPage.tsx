import {Col, Row} from "react-bootstrap";
import useTitle from "./useTitle.ts";

export function DashboardPage() {
    useTitle("Dashboard");
    return (
        <Row className="me-0">
            <Col xs={12} md={4} lg={3} xl={2} className="pe-0">
                <div className="pt-3 bg-body-secondary border-end border-bottom vh-100 overflow-y-scroll">
                    <code># TODO: some actions? </code>
                </div>
            </Col>
            <Col xs={12} md={8} lg={6} xl={8}>
                <div className="pt-3">
                    <div className="fs-5">Feed</div>
                    <code># TODO: recent activity? </code>
                </div>
            </Col>
            <Col xs={12} md={12} lg={3} xl={2} className="me-0 ps-4 ps-lg-0">
                <div className="py-3 d-flex flex-column gap-3">
                    <div className="p-3 bg-body-secondary border rounded-2">
                        <div className="small">Latest</div>
                    </div>
                    <div className="p-3 bg-body-secondary border rounded-2">
                        <div className="small">Discover</div>
                    </div>
                    <div className="p-3 bg-body-secondary border rounded-2">
                        <div className="small">Explore</div>
                    </div>
                </div>
            </Col>
        </Row>
    );
}
