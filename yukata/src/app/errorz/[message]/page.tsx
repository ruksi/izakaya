export default function Errorz({params}: {params: {message: string}}) {
    throw new Error(params.message);
}
