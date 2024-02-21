# `🍙 onigiri` Frontend

Vite + React + React Router

## Development

```bash
npm install
npm run dev
```

## "Production"

Requirements:

- a [Railway](https://railway.app/) project
- one of the backends running in this repository
- this repository or a fork linked to the Railway project

Steps:

- Add a new service from "GitHub Repository"
    - Select this repository
    - Settings:
        - Service name: `🍙 onigiri`
        - Click "Add Root Directory" and edit it: `/onigiri`
    - Variables:
        - `VITE_BACKEND_URL`: `https://<backend-public-url-you-have>`
    - Click "Deploy"

Next Steps:

- Configure public networking for `🍙 onigiri` after it runs
- Add the `🍙 onigiri` public URL to your backend env variables
