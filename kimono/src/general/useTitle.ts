import {useEffect} from "react"

function useTitle(title: string) {
    useEffect(() => {
        document.title = title;
    }, [title]);
    useEffect(() => () => {
        document.title = "Izakaya";
    }, [])
}

export default useTitle;
