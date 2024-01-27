import {useEffect} from "react"

function useTitle(title: string) {
    useEffect(() => {
        document.title = title;
    }, [title]);
    useEffect(() => () => {
        document.title = "Ryokan";
    }, [])
}

export default useTitle;
