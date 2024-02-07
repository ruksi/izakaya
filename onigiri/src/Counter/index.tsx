import Button from "react-bootstrap/Button";
import Stack from "react-bootstrap/esm/Stack";
import {useAppDispatch, useAppSelector} from "../hooks.ts";
import counterSlice from "./slice.ts";


export function Counter() {
    const count = useAppSelector((state) => state.counter.value);
    const dispatch = useAppDispatch();
    const {decrement, increment, incrementByAmount} = counterSlice.actions;
    return (
        <div>
            <Stack direction="horizontal" gap={2}>
                <Button variant="secondary" onClick={() => dispatch(incrementByAmount(-5))}>-5</Button>
                <Button variant="secondary" onClick={() => dispatch(decrement())}>-1</Button>
                <span className="px-3 text-center fw-bold fs-3" style={{minWidth: "5rem"}}>{count}</span>
                <Button variant="secondary" onClick={() => dispatch(increment())}>+1</Button>
                <Button variant="secondary" onClick={() => dispatch(incrementByAmount(5))}>+5</Button>
            </Stack>
        </div>
    )
}
