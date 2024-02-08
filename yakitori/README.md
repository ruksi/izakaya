# `🍗 yakitori` Frontend

Next 14 + App Router

# Development

```bash
npm install
npm run dev
```

## "Production"

Requirements:

* a [Railway](https://railway.app/) project
* one of the backends running in this repository
* this repository or a fork linked to the Railway project

Steps:

* Add a new service from "GitHub Repository"
    * Select this repository
    * Settings:
        * Service name: `🍗 yakitori`
        * Click "Add Root Directory" and edit it: `/yakitori`
    * Variables:
        * `NEXT_PUBLIC_BACKEND_URL` = `https://<backend-public-url-you-have>`
        * `NEXT_PUBLIC_SELF_URL` = `https://<the-public-url-you-will-use>`
    * Click "Deploy"

Next Steps:

* Configure public networking for `🍗 yakitori` after it runs
* Add the `🍗 yakitori` public URL to your backend env variables
